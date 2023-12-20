import { test, expect } from "vitest"
import { observeAtom } from "../../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../../common/util/timer/helper"

import { mockSingleBoardStore } from "../../../../../common/util/board/input/test_helper"
import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../../common/util/clock/mock"
import { mockSecureServerURL } from "../../../../sign/get_script_path/detail/mock"
import { mockResetPasswordShell } from "./detail/mock"
import { initMemoryDB } from "../../../../../common/util/repository/detail/memory"

import { authTicketRepositoryConverter } from "../../../../ticket/kernel/convert"
import { convertCheckRemote } from "../../../../ticket/authenticate/convert"
import { convertDB } from "../../../../../common/util/repository/detail/convert"

import { Clock } from "../../../../../common/util/clock/infra"
import { ResetPasswordRemote, ResetPasswordRemoteResult } from "./infra"
import { AuthTicketRepository, AuthTicketRepositoryValue } from "../../../../ticket/kernel/infra"
import { CheckAuthTicketRemote } from "../../../../ticket/authenticate/infra"
import { SingleBoardStore } from "../../../../../common/util/board/input/infra"

import { initResetPasswordAction, ResetPasswordAction } from "./action"

// テスト開始時刻
const START_AT = new Date("2020-01-01 10:00:00")

// renew 設定時刻 : succeed-to-start-continuous-renew でこの時刻に移行
const CONTINUOUS_RENEW_START_AT = new Date("2020-01-01 10:00:01")

// renew ごとに次の時刻に移行
const CONTINUOUS_RENEW_AT = [new Date("2020-01-01 10:01:00"), new Date("2020-01-01 11:00:00")]

const VALID_LOGIN = { newPassword: "password" } as const

test("submit valid login-id and password", async () => {
    const { clock, action, store } = standard()

    action.state.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    const result = observeAtom(action.state)

    store.newPassword.set(VALID_LOGIN.newPassword)

    await action.submit()

    expect(result()).toEqual([
        { type: "try-to-reset", hasTakenLongtime: false },
        {
            type: "try-to-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "required-to-login", continue: false },
    ])
})

test("submit valid login-id and password; with take longtime", async () => {
    // wait for take longtime timeout
    const { clock, action, store } = takeLongtime()

    action.state.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    const result = observeAtom(action.state)

    store.newPassword.set(VALID_LOGIN.newPassword)

    await action.submit()

    expect(result()).toEqual([
        { type: "try-to-reset", hasTakenLongtime: false },
        { type: "try-to-reset", hasTakenLongtime: true },
        {
            type: "try-to-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "required-to-login", continue: false },
    ])
})

test("submit without fields", async () => {
    const { action } = standard()

    const result = observeAtom(action.state)

    await action.submit()

    expect(result()).toEqual([])
})

test("submit without resetToken", async () => {
    const { action, store } = noResetToken()

    const result = observeAtom(action.state)

    store.newPassword.set(VALID_LOGIN.newPassword)

    await action.submit()

    expect(result()).toEqual([{ type: "failed-to-reset", err: { type: "empty-reset-token" } }])
})
test("submit with empty resetToken", async () => {
    const { action, store } = emptyResetToken()

    const result = observeAtom(action.state)

    store.newPassword.set(VALID_LOGIN.newPassword)

    await action.submit()

    expect(result()).toEqual([{ type: "failed-to-reset", err: { type: "empty-reset-token" } }])
})

test("reset", () => {
    const { action, store } = standard()

    store.newPassword.set(VALID_LOGIN.newPassword)
    action.reset()

    expect(store.newPassword.get()).toEqual("")
})

test("load error", async () => {
    const { action } = standard()

    const result = observeAtom(action.state)

    await action.loadError({ type: "infra-error", err: "load error" })

    expect(result()).toEqual([
        { type: "load-error", err: { type: "infra-error", err: "load error" } },
    ])
})

function standard() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const resource = initResource(
        standard_URL(),
        standard_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...resource }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const resource = initResource(
        standard_URL(),
        takeLongtime_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...resource }
}
function noResetToken() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    return initResource(
        noResetToken_URL(),
        standard_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )
}
function emptyResetToken() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    return initResource(
        emptyResetToken_URL(),
        standard_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )
}

function initResource(
    currentURL: URL,
    resetRemote: ResetPasswordRemote,
    renewRemote: CheckAuthTicketRemote,
    clock: Clock,
): Readonly<{
    action: ResetPasswordAction
    store: Readonly<{
        newPassword: SingleBoardStore
    }>
}> {
    const ticketRepository = standard_ticketRepository()

    const action = initResetPasswordAction({
        infra: {
            ticketRepository,
            renewRemote,
            resetRemote,
            clock,
        },
        shell: mockResetPasswordShell(currentURL),
        config: {
            continuousRenewInterval: { interval_millisecond: 64 },
            ticketExpire: { expire_millisecond: 500 },
            takeLongtimeThreshold: { wait_millisecond: 32 },
            secureServerURL: mockSecureServerURL("https://secure.example.com"),
        },
    })

    const store = {
        newPassword: mockSingleBoardStore(action.newPassword.input),
    }

    return { action, store }
}

function standard_URL(): URL {
    return new URL("https://example.com/index.html?-password-reset-token=reset-token")
}
function noResetToken_URL(): URL {
    return new URL("https://example.com/index.html")
}
function emptyResetToken_URL(): URL {
    return new URL("https://example.com/index.html?-password-reset-token=")
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        granted: ["permission"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}

function standard_resetRemote(clock: Clock): ResetPasswordRemote {
    return async () => standard_resetPasswordRemoteResult(clock)
}
function takeLongtime_resetRemote(clock: Clock): ResetPasswordRemote {
    return async () =>
        ticker({ wait_millisecond: 64 }, () => standard_resetPasswordRemoteResult(clock))
}
function standard_resetPasswordRemoteResult(clock: Clock): ResetPasswordRemoteResult {
    return {
        success: true,
        value: convertCheckRemote(clock, ["permission"]),
    }
}

function standard_renewRemote(clock: Clock, clockPubSub: ClockPubSub): CheckAuthTicketRemote {
    let count = 0
    return async () => {
        if (count > 1) {
            // 最初の 2回だけ renew して、あとは renew を cancel するための unauthorized
            return { success: false, err: { type: "unauthorized" } }
        }

        // 現在時刻を動かす
        const nextTime = CONTINUOUS_RENEW_AT[count]
        setTimeout(() => clockPubSub.update(nextTime))

        count++
        return {
            success: true,
            value: convertCheckRemote(clock, ["permission"]),
        }
    }
}
