import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"
import { toApplicationView } from "../../../../ui/vendor/getto-application/action/helper"
import { ticker } from "../../../z_lib/ui/timer/helper"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { mockGetScriptPathShell, mockSecureServerURL } from "../../sign/get_script_path/init/mock"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { convertDB } from "../../../z_lib/ui/repository/init/convert"
import { authTicketRepositoryConverter, convertAuthRemote } from "../kernel/convert"

import { Clock } from "../../../z_lib/ui/clock/infra"
import { WaitTime } from "../../../z_lib/ui/config/infra"
import {
    AuthTicketRepository,
    AuthTicketRepositoryValue,
    RenewAuthTicketRemote,
} from "../kernel/infra"

import { ApplicationView } from "../../../../ui/vendor/getto-application/action/action"
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

describe("CheckAuthTicket", () => {
    test("instant load", async () => {
        const { clock, view } = instantLoadable()
        const resource = view.resource

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => resource.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-instant-load",
                    scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
                },
            ])
        })

        clock.update(CONTINUOUS_RENEW_START_AT)

        await runner(() => resource.succeedToInstantLoad()).then((stack) => {
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
        const { clock, view } = instantLoadable()
        const resource = view.resource

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => resource.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-instant-load",
                    scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
                },
            ])
        })

        clock.update(CONTINUOUS_RENEW_START_AT)

        await runner(() => resource.failedToInstantLoad()).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-renew" },
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
        const { clock, view } = standard()
        const resource = view.resource

        resource.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => resource.ignitionState).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-renew" },
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
        const { clock, view } = takeLongtime()
        const resource = view.resource

        resource.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => resource.ignitionState).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-renew" },
                { type: "take-longtime-to-renew" },
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
        const { view } = noStored()
        const resource = view.resource

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => resource.ignitionState).then((stack) => {
            expect(stack).toEqual([{ type: "required-to-login" }])
        })
    })

    test("load error", async () => {
        const { view } = standard()
        const resource = view.resource

        const runner = setupActionTestRunner(resource.subscriber)

        const err: LoadScriptError = { type: "infra-error", err: "load error" }

        await runner(() => resource.loadError(err)).then((stack) => {
            expect(stack).toEqual([{ type: "load-error", err }])
        })
    })

    test("terminate", async () => {
        const { view } = standard()

        const runner = setupActionTestRunner(view.resource.subscriber)

        await runner(() => {
            view.terminate()
            return view.resource.ignitionState
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        standard_ticketRepository(),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, view }
}
function instantLoadable() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT_INSTANT_LOAD_AVAILABLE, clockPubSub)
    const view = initView(
        standard_ticketRepository(),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, view }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(standard_ticketRepository(), wait_renewRemote(clock, clockPubSub), clock)
    return { clock: clockPubSub, view }
}
function noStored() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        noStored_ticketRepository(),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )
    return { view }
}

function initView(
    ticketRepository: AuthTicketRepository,
    renewRemote: RenewAuthTicketRemote,
    clock: Clock,
): ApplicationView<CheckAuthTicketAction> {
    return toApplicationView(
        initCheckAuthTicketAction({
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
                takeLongtimeThreshold: { delay_millisecond: 32 },
                secureServerURL: mockSecureServerURL("https://secure.example.com"),
            },
        }),
    )
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: STORED_LAST_AUTH_AT,
        roles: ["role"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}
function noStored_ticketRepository(): AuthTicketRepository {
    return initMemoryDB<AuthTicket>()
}

function standard_renewRemote(clock: Clock, clockPubSub: ClockPubSub): RenewAuthTicketRemote {
    return renewRemote(clock, clockPubSub, { wait_millisecond: 0 })
}
function wait_renewRemote(clock: Clock, clockPubSub: ClockPubSub): RenewAuthTicketRemote {
    // wait for take longtime timeout
    return renewRemote(clock, clockPubSub, { wait_millisecond: 64 })
}
function renewRemote(
    clock: Clock,
    clockPubSub: ClockPubSub,
    waitTime: WaitTime,
): RenewAuthTicketRemote {
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
                value: convertAuthRemote(clock, { roles: ["role"] }),
            }
        })
}
