import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../z_lib/ui/timer/helper"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { mockGetScriptPathShell, mockSecureServerURL } from "../../sign/get_script_path/init/mock"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { convertDB } from "../../../z_lib/ui/repository/init/convert"
import { authTicketRepositoryConverter } from "../kernel/convert"
import { convertCheckRemote } from "../check/convert"

import { Clock } from "../../../z_lib/ui/clock/infra"
import { WaitTime } from "../../../z_lib/ui/config/infra"
import { AuthTicketRepository, AuthTicketRepositoryValue } from "../kernel/infra"
import { CheckAuthTicketRemote } from "./infra"

import { CheckAuthTicketAction, initCheckAuthTicketAction } from "./action"

import { LoadScriptError } from "../../sign/get_script_path/data"
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

    const runner = setupActionTestRunner(action.state)

    await runner(() => action.state.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "try-to-instant-load",
                scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
            },
        ])
    })

    clock.update(CONTINUOUS_RENEW_START_AT)

    await runner(() => action.succeedToInstantLoad()).then((stack) => {
        expect(stack).toEqual([
            { type: "succeed-to-start-continuous-renew", continue: true },
            { type: "succeed-to-renew", continue: true },
            { type: "succeed-to-renew", continue: true },
            { type: "succeed-to-renew", continue: true },
            { type: "required-to-login", continue: false },
        ])
    })
})

test("instant load failed", async () => {
    const { clock, action } = instantLoadable()

    const runner = setupActionTestRunner(action.state)

    await runner(() => action.state.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "try-to-instant-load",
                scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
            },
        ])
    })

    clock.update(CONTINUOUS_RENEW_START_AT)

    await runner(() => action.failedToInstantLoad()).then((stack) => {
        expect(stack).toEqual([
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
})

test("renew stored credential", async () => {
    const { clock, action } = standard()

    action.state.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    const runner = setupActionTestRunner(action.state)

    await runner(() => action.state.ignitionState).then((stack) => {
        expect(stack).toEqual([
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
})

test("renew stored credential; take long time", async () => {
    // wait for take longtime timeout
    const { clock, action } = takeLongtime()

    action.state.subscribe((state) => {
        switch (state.type) {
            case "try-to-load":
                clock.update(CONTINUOUS_RENEW_START_AT)
                break
        }
    })

    const runner = setupActionTestRunner(action.state)

    await runner(() => action.state.ignitionState).then((stack) => {
        expect(stack).toEqual([
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
})

test("renew without stored credential", async () => {
    // empty credential
    const { action } = noStored()

    const runner = setupActionTestRunner(action.state)

    await runner(() => action.state.ignitionState).then((stack) => {
        expect(stack).toEqual([{ type: "required-to-login" }])
    })
})

test("load error", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.state)

    const err: LoadScriptError = { type: "infra-error", err: "load error" }

    await runner(() => action.loadError(err)).then((stack) => {
        expect(stack).toEqual([{ type: "load-error", err }])
    })
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
): CheckAuthTicketAction {
    return initCheckAuthTicketAction({
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
        grantedRoles: ["role"],
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
                value: convertCheckRemote(clock, ["role"]),
            }
        })
}
