import { test, expect } from "vitest"
import { observeAtom } from "../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../common/util/timer/helper"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../common/util/clock/mock"
import { mockGetScriptPathShell, mockSecureServerURL } from "../../sign/get_script_path/detail/mock"
import { initMemoryDB } from "../../../common/util/repository/detail/memory"

import { convertDB } from "../../../common/util/repository/detail/convert"
import { authTicketRepositoryConverter } from "../kernel/convert"
import { convertCheckRemote } from "./convert"

import { Clock } from "../../../common/util/clock/infra"
import { WaitTime } from "../../../common/util/config/infra"
import { AuthTicketRepository, AuthTicketRepositoryValue } from "../kernel/infra"
import { CheckAuthTicketRemote } from "./infra"

import { AuthenticateWithTokenAction, initAuthenticateWithTokenAction } from "./action"

import { AuthTicket } from "../kernel/data"

// last auth at : テスト開始時刻と expire 設定によって instant load の可否が決まる
const STORED_LAST_AUTH_AT = new Date("2020-01-01 10:00:00").toISOString()

// テスト開始時刻
const START_AT_INSTANT_LOAD_AVAILABLE = new Date("2020-01-01 10:00:10")
const START_AT = new Date("2020-01-01 10:00:30")

// renew 設定時刻 : succeed-to-start-continuous-renew でこの時刻に移行
const CONTINUOUS_RENEW_START_AT = new Date("2020-01-01 10:00:40")

// renew ごとに次の時刻に移行
const CONTINUOUS_RENEW_AT = [
    new Date("2020-01-01 10:01:00"),
    new Date("2020-01-01 10:02:00"),
    new Date("2020-01-01 11:00:00"),
]

test("instant load", async () => {
    const { clock, action } = instantLoadable()

    const result = observeAtom(action.state)

    await action.state.ignitionState

    clock.update(CONTINUOUS_RENEW_START_AT)

    await action.succeedToInstantLoad()

    expect(result()).toEqual([
        {
            type: "try-to-instant-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "succeed-to-start-continuous-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "required-to-login", continue: false },
    ])
})

test("instant load failed", async () => {
    const { clock, action } = instantLoadable()

    const result = observeAtom(action.state)

    await action.state.ignitionState

    clock.update(CONTINUOUS_RENEW_START_AT)

    await action.failedToInstantLoad()

    expect(result()).toEqual([
        {
            type: "try-to-instant-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "try-to-renew", hasTakenLongtime: false },
        {
            type: "try-to-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "required-to-login", continue: false },
    ])
})

test("renew stored credential", async () => {
    const { clock, action } = standard()

    const result = observeAtom(action.state)

    action.state.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    await action.state.ignitionState

    expect(result()).toEqual([
        { type: "try-to-renew", hasTakenLongtime: false },
        {
            type: "try-to-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "required-to-login", continue: false },
    ])
})

test("renew stored credential; take long time", async () => {
    // wait for take longtime timeout
    const { clock, action } = takeLongtime()

    const result = observeAtom(action.state)

    action.state.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    await action.state.ignitionState

    expect(result()).toEqual([
        { type: "try-to-renew", hasTakenLongtime: false },
        { type: "try-to-renew", hasTakenLongtime: true },
        {
            type: "try-to-load",
            scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
        },
        { type: "succeed-to-renew", continue: true },
        { type: "succeed-to-renew", continue: true },
        { type: "required-to-login", continue: false },
    ])
})

test("renew without stored credential", async () => {
    // empty credential
    const { action } = noStored()

    const result = observeAtom(action.state)

    await action.state.ignitionState

    expect(result()).toEqual([{ type: "required-to-login" }])
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
    const action = initAction(
        standard_ticketRepository(),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, action }
}
function instantLoadable() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT_INSTANT_LOAD_AVAILABLE, clockPubSub)
    const action = initAction(
        standard_ticketRepository(),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, action }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const action = initAction(
        standard_ticketRepository(),
        wait_renewRemote(clock, clockPubSub),
        clock,
    )
    return { clock: clockPubSub, action }
}
function noStored() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const action = initAction(
        noStored_ticketRepository(),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )
    return { action }
}

function initAction(
    ticketRepository: AuthTicketRepository,
    renewRemote: CheckAuthTicketRemote,
    clock: Clock,
): AuthenticateWithTokenAction {
    return initAuthenticateWithTokenAction({
        infra: {
            ticketRepository,
            renewRemote,
            clock,
        },
        shell: {
            ...mockGetScriptPathShell(new URL("https://example.com/index.html")),
        },
        config: {
            continuousRenewInterval: { interval_millisecond: 128 },
            ticketExpire: { expire_millisecond: 1 * 1000 },
            instantLoadExpire: { expire_millisecond: 20 * 1000 },
            takeLongtimeThreshold: { wait_millisecond: 32 },
            secureServerURL: mockSecureServerURL("https://secure.example.com"),
        },
    })
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: STORED_LAST_AUTH_AT,
        granted: ["permission"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}
function noStored_ticketRepository(): AuthTicketRepository {
    return initMemoryDB<AuthTicket>()
}

function standard_renewRemote(clock: Clock, clockPubSub: ClockPubSub): CheckAuthTicketRemote {
    return renewRemote(clock, clockPubSub, { wait_millisecond: 0 })
}
function wait_renewRemote(clock: Clock, clockPubSub: ClockPubSub): CheckAuthTicketRemote {
    // wait for take longtime timeout
    return renewRemote(clock, clockPubSub, { wait_millisecond: 64 })
}
function renewRemote(
    clock: Clock,
    clockPubSub: ClockPubSub,
    waitTime: WaitTime,
): CheckAuthTicketRemote {
    let count = 0
    return async () =>
        ticker(waitTime, () => {
            if (count > 2) {
                // 最初の 3回だけ renew して、あとは renew を cancel するための unauthorized
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
        })
}
