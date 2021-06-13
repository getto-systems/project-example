import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"

import {
    ClockPubSub,
    mockClock,
    mockClockPubSub,
} from "../../../../../ui/vendor/getto-application/infra/clock/mock"
import { mockRepository } from "../../../../../ui/vendor/getto-application/infra/repository/mock"
import { mockRemotePod } from "../../../../../ui/vendor/getto-application/infra/remote/mock"

import { mockGetScriptPathDetecter } from "../../../_ui/common/secure/get_script_path/mock"

import { convertRepository } from "../../../../../ui/vendor/getto-application/infra/repository/helper"
import { initCheckAuthTicketView } from "./impl"
import { initCheckAuthTicketCoreAction, initCheckAuthTicketCoreMaterial } from "./core/impl"

import { Clock } from "../../../../../ui/vendor/getto-application/infra/clock/infra"
import { WaitTime } from "../../../../../ui/vendor/getto-application/infra/config/infra"
import { AuthnRepositoryValue, AuthzRepositoryPod, AuthzRepositoryValue } from "../kernel/infra"
import { AuthnRepositoryPod, RenewAuthTicketRemotePod } from "../kernel/infra"

import { CheckAuthTicketView } from "./resource"

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
    const view = initView(
        standard_authn(),
        standard_authz(),
        standard_renew(clockPubSub),
        mockClock(START_AT, clockPubSub),
    )

    return { clock: clockPubSub, view }
}
function instantLoadable() {
    const clockPubSub = mockClockPubSub()
    const view = initView(
        standard_authn(),
        standard_authz(),
        standard_renew(clockPubSub),
        mockClock(START_AT_INSTANT_LOAD_AVAILABLE, clockPubSub),
    )

    return { clock: clockPubSub, view }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const view = initView(
        standard_authn(),
        standard_authz(),
        wait_renew(clockPubSub),
        mockClock(START_AT, clockPubSub),
    )
    return { clock: clockPubSub, view }
}
function noStored() {
    const clockPubSub = mockClockPubSub()
    const view = initView(
        noStored_authn(),
        noStored_authz(),
        standard_renew(clockPubSub),
        mockClock(START_AT, clockPubSub),
    )
    return { view }
}

function initView(
    authn: AuthnRepositoryPod,
    authz: AuthzRepositoryPod,
    renew: RenewAuthTicketRemotePod,
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

function standard_renew(clock: ClockPubSub): RenewAuthTicketRemotePod {
    return renewPod(clock, { wait_millisecond: 0 })
}
function wait_renew(clock: ClockPubSub): RenewAuthTicketRemotePod {
    // wait for take longtime timeout
    return renewPod(clock, { wait_millisecond: 64 })
}
function renewPod(clock: ClockPubSub, waitTime: WaitTime): RenewAuthTicketRemotePod {
    let count = 0
    return mockRemotePod(() => {
        if (count > 2) {
            // 最初の 3回だけ renew して、あとは renew を cancel するための unauthorized
            return { success: false, err: { type: "unauthorized" } }
        }

        // 現在時刻を動かす
        const nextTime = CONTINUOUS_RENEW_AT[count]
        setTimeout(() => clock.update(nextTime))

        count++
        return {
            success: true,
            value: {
                roles: ["role"],
            },
        }
    }, waitTime)
}
