import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_details/_ui/timer/helper"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../z_details/_ui/clock/mock"
import { mockRepository } from "../../../../z_details/_ui/repository/mock"

import { mockGetScriptPathDetecter } from "../../../_ui/common/secure/get_script_path/mock"

import { convertRepository } from "../../../../z_details/_ui/repository/helper"
import { initCheckAuthTicketView } from "./impl"
import { initCheckAuthTicketCoreAction, initCheckAuthTicketCoreMaterial } from "./core/impl"

import { Clock } from "../../../../z_details/_ui/clock/infra"
import { WaitTime } from "../../../../z_details/_ui/config/infra"
import {
    AuthnRepositoryPod,
    AuthnRepositoryValue,
    AuthzRepositoryPod,
    AuthzRepositoryValue,
    RenewAuthTicketRemote,
} from "../kernel/infra"

import { CheckAuthTicketView } from "./resource"

import { authRemoteConverter } from "../kernel/converter"

import { LoadScriptError } from "../../../_ui/common/secure/get_script_path/data"

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

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => resource.core.ignite()).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-instant-load",
                    scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
                },
            ])
        })

        clock.update(CONTINUOUS_RENEW_START_AT)

        await runner(() => resource.core.succeedToInstantLoad()).then((stack) => {
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

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => resource.core.ignite()).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-instant-load",
                    scriptPath: { valid: true, value: "https://secure.example.com/index.js" },
                },
            ])
        })

        clock.update(CONTINUOUS_RENEW_START_AT)

        await runner(() => resource.core.failedToInstantLoad()).then((stack) => {
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

        resource.core.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => resource.core.ignite()).then((stack) => {
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

        resource.core.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => resource.core.ignite()).then((stack) => {
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

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => resource.core.ignite()).then((stack) => {
            expect(stack).toEqual([{ type: "required-to-login" }])
        })
    })

    test("load error", async () => {
        const { view } = standard()
        const resource = view.resource

        const runner = setupActionTestRunner(resource.core.subscriber)

        const err: LoadScriptError = { type: "infra-error", err: "load error" }

        await runner(() => resource.core.loadError(err)).then((stack) => {
            expect(stack).toEqual([{ type: "load-error", err }])
        })
    })

    test("terminate", async () => {
        const { view } = standard()

        const runner = setupActionTestRunner(view.resource.core.subscriber)

        await runner(() => {
            view.terminate()
            return view.resource.core.ignite()
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
        standard_authn(),
        standard_authz(),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, view }
}
function instantLoadable() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT_INSTANT_LOAD_AVAILABLE, clockPubSub)
    const view = initView(
        standard_authn(),
        standard_authz(),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, view }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(standard_authn(), standard_authz(), wait_renew(clock, clockPubSub), clock)
    return { clock: clockPubSub, view }
}
function noStored() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        noStored_authn(),
        noStored_authz(),
        standard_renew(clock, clockPubSub),
        clock,
    )
    return { view }
}

function initView(
    authn: AuthnRepositoryPod,
    authz: AuthzRepositoryPod,
    renew: RenewAuthTicketRemote,
    clock: Clock,
): CheckAuthTicketView {
    const currentURL = new URL("https://example.com/index.html")
    const getScriptPathDetecter = mockGetScriptPathDetecter(currentURL)
    return initCheckAuthTicketView(
        initCheckAuthTicketCoreAction(
            initCheckAuthTicketCoreMaterial(
                {
                    check: {
                        authn,
                        authz,
                        renew,
                        config: {
                            instantLoadExpire: { expire_millisecond: 20 * 1000 },
                            takeLongtimeThreshold: { delay_millisecond: 32 },
                        },
                        clock,
                    },
                    startContinuousRenew: {
                        authn,
                        authz,
                        renew,
                        config: {
                            interval: { interval_millisecond: 128 },
                            authnExpire: { expire_millisecond: 1 * 1000 },
                        },
                        clock,
                    },
                    getSecureScriptPath: {
                        config: {
                            secureServerURL: "https://secure.example.com",
                        },
                    },
                },
                getScriptPathDetecter,
            ),
        ),
    )
}

function standard_authn(): AuthnRepositoryPod {
    const db = mockRepository<AuthnRepositoryValue>()
    db.set({
        authAt: STORED_LAST_AUTH_AT,
    })
    return convertRepository(db)
}
function noStored_authn(): AuthnRepositoryPod {
    return convertRepository(mockRepository<AuthnRepositoryValue>())
}

function standard_authz(): AuthzRepositoryPod {
    const db = mockRepository<AuthzRepositoryValue>()
    db.set({
        roles: ["role"],
    })
    return convertRepository(db)
}
function noStored_authz(): AuthzRepositoryPod {
    return convertRepository(mockRepository<AuthzRepositoryValue>())
}

function standard_renew(clock: Clock, clockPubSub: ClockPubSub): RenewAuthTicketRemote {
    return renewRemote(clock, clockPubSub, { wait_millisecond: 0 })
}
function wait_renew(clock: Clock, clockPubSub: ClockPubSub): RenewAuthTicketRemote {
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
                value: authRemoteConverter(clock, {
                    roles: ["role"],
                }),
            }
        })
}
