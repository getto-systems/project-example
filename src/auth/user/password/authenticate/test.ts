import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { AuthenticatePasswordAction, initAuthenticatePasswordAction } from "./action"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../z_lib/ui/clock/mock"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import {
    mockGetScriptPathShell,
    mockSecureServerURL,
} from "../../../sign/get_script_path/init/mock"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"
import { convertDB } from "../../../../z_lib/ui/repository/init/convert"

import { Clock } from "../../../../z_lib/ui/clock/infra"
import { AuthenticatePasswordRemote, AuthenticatePasswordRemoteResult } from "./infra"
import { AuthTicketRepository, AuthTicketRepositoryValue } from "../../../ticket/kernel/infra"
import { CheckAuthTicketRemote } from "../../../ticket/check/infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { authTicketRepositoryConverter } from "../../../ticket/kernel/convert"
import { convertCheckRemote } from "../../../ticket/check/convert"

import { LoadScriptError } from "../../../sign/get_script_path/data"

// テスト開始時刻
const START_AT = new Date("2020-01-01 10:00:00")

// renew 設定時刻 : succeed-to-start-continuous-renew でこの時刻に移行
const CONTINUOUS_RENEW_START_AT = new Date("2020-01-01 10:00:01")

// renew ごとに次の時刻に移行
const CONTINUOUS_RENEW_AT = [new Date("2020-01-01 10:01:00"), new Date("2020-01-01 11:00:00")]

const VALID_LOGIN = { loginId: "login-id", password: "password" } as const

test("submit valid login-id and password", async () => {
    const { clock, action, store } = standard()

    action.subscriber.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        store.loginId.set(VALID_LOGIN.loginId)
        store.password.set(VALID_LOGIN.password)

        return action.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try-to-login", hasTakenLongtime: false },
            {
                type: "try-to-load",
                scriptPath: {
                    valid: true,
                    value: "https://secure.example.com/index.js",
                },
            },
            { type: "succeed-to-renew", continue: true },
            { type: "succeed-to-renew", continue: true },
            { type: "required-to-login", continue: false },
        ])
    })
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { clock, action, store } = takeLongtime_elements()

    action.subscriber.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    const runner = setupActionTestRunner(action.subscriber)

    await runner(() => {
        store.loginId.set(VALID_LOGIN.loginId)
        store.password.set(VALID_LOGIN.password)

        return action.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try-to-login", hasTakenLongtime: false },
            { type: "try-to-login", hasTakenLongtime: true },
            {
                type: "try-to-load",
                scriptPath: {
                    valid: true,
                    value: "https://secure.example.com/index.js",
                },
            },
            { type: "succeed-to-renew", continue: true },
            { type: "succeed-to-renew", continue: true },
            { type: "required-to-login", continue: false },
        ])
    })
})

test("submit without fields", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(() => action.submit()).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("clear", () => {
    const { action, store } = standard()

    store.loginId.set(VALID_LOGIN.loginId)
    store.password.set(VALID_LOGIN.password)
    action.clear()

    expect(store.loginId.get()).toEqual("")
    expect(store.password.get()).toEqual("")
})

test("load error", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    const err: LoadScriptError = { type: "infra-error", err: "load error" }

    await runner(() => action.loadError(err)).then((stack) => {
        expect(stack).toEqual([{ type: "load-error", err }])
    })
})

function standard() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const resource = initResource(
        standard_authenticate(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...resource }
}
function takeLongtime_elements() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const resource = initResource(
        takeLongtime_authenticate(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...resource }
}

function initResource(
    authenticateRemote: AuthenticatePasswordRemote,
    renewRemote: CheckAuthTicketRemote,
    clock: Clock,
): Readonly<{
    action: AuthenticatePasswordAction
    store: Readonly<{
        loginId: BoardValueStore
        password: BoardValueStore
    }>
}> {
    const currentURL = new URL("https://example.com/index.html")

    const ticketRepository = standard_ticketRepository()

    const action = initAuthenticatePasswordAction({
        infra: {
            ticketRepository,
            renewRemote,
            authenticateRemote,
            clock,
        },
        shell: {
            ...mockGetScriptPathShell(currentURL),
        },
        config: {
            continuousRenewInterval: { interval_millisecond: 128 },
            ticketExpire: { expire_millisecond: 500 },
            takeLongtimeThreshold: { wait_millisecond: 32 },
            secureServerURL: mockSecureServerURL("https://secure.example.com"),
        },
    })

    const store = {
        loginId: mockBoardValueStore(action.loginId.input),
        password: mockBoardValueStore(action.password.input),
    }

    return { action, store }
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        grantedRoles: ["role"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}

function standard_authenticate(clock: Clock): AuthenticatePasswordRemote {
    return async () => standard_authenticateRemoteResult(clock)
}
function takeLongtime_authenticate(clock: Clock): AuthenticatePasswordRemote {
    return async () =>
        ticker({ wait_millisecond: 64 }, () => standard_authenticateRemoteResult(clock))
}
function standard_authenticateRemoteResult(clock: Clock): AuthenticatePasswordRemoteResult {
    return {
        success: true,
        value: convertCheckRemote(clock, ["role"]),
    }
}

function standard_renew(clock: Clock, clockPubSub: ClockPubSub): CheckAuthTicketRemote {
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
            value: convertCheckRemote(clock, ["role"]),
        }
    }
}
