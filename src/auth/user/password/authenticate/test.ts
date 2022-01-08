import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../z_lib/ui/clock/mock"
import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/init/mock"
import { mockGetScriptPathShell, mockSecureServerURL } from "../../../sign/get_script_path/init/mock"

import { Clock } from "../../../../z_lib/ui/clock/infra"
import { AuthenticatePasswordRemote, AuthenticatePasswordRemoteResult } from "./infra"
import {
    AuthTicketRepository,
    AuthTicketRepositoryValue,
    RenewAuthTicketRemote,
} from "../../../ticket/kernel/infra"
import { BoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"

import { authTicketRepositoryConverter, convertAuthRemote } from "../../../ticket/kernel/convert"

import { LoadScriptError } from "../../../sign/get_script_path/data"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"
import { convertDB } from "../../../../z_lib/ui/repository/init/convert"
import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { AuthenticatePasswordAction, initAuthenticatePasswordAction } from "./action"

// テスト開始時刻
const START_AT = new Date("2020-01-01 10:00:00")

// renew 設定時刻 : succeed-to-start-continuous-renew でこの時刻に移行
const CONTINUOUS_RENEW_START_AT = new Date("2020-01-01 10:00:01")

// renew ごとに次の時刻に移行
const CONTINUOUS_RENEW_AT = [new Date("2020-01-01 10:01:00"), new Date("2020-01-01 11:00:00")]

const VALID_LOGIN = { loginID: "login-id", password: "password" } as const

describe("AuthenticatePassword", () => {
    test("submit valid login-id and password", async () => {
        const { clock, view, store } = standard()
        const resource = view.resource

        resource.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(async () => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            store.password.set(markBoardValue(VALID_LOGIN.password))

            return resource.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-login" },
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
        const { clock, view, store } = takeLongtime_elements()
        const resource = view.resource

        resource.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            store.password.set(markBoardValue(VALID_LOGIN.password))

            return resource.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-login" },
                { type: "take-longtime-to-login" },
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
        const { view } = standard()
        const resource = view.resource

        const runner = setupActionTestRunner(resource.subscriber)

        await runner(() => resource.submit()).then((stack) => {
            expect(stack).toEqual([{ type: "failed-to-login", err: { type: "validation-error" } }])
        })
    })

    test("clear", () => {
        const { view, store } = standard()
        const resource = view.resource

        store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
        store.password.set(markBoardValue(VALID_LOGIN.password))
        resource.clear()

        expect(store.loginID.get()).toEqual("")
        expect(store.password.get()).toEqual("")
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
        const resource = view.resource

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                resource.subscriber.subscribe(handler)
                resource.validate.subscriber.subscribe(handler)
                resource.loginID.validate.subscriber.subscribe(handler)
                resource.password.validate.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            view.terminate()
            return resource.submit()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(standard_authenticate(clock), standard_renew(clock, clockPubSub), clock)

    return { clock: clockPubSub, ...view }
}
function takeLongtime_elements() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        takeLongtime_authenticate(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...view }
}

function initView(
    authenticateRemote: AuthenticatePasswordRemote,
    renewRemote: RenewAuthTicketRemote,
    clock: Clock,
): Readonly<{
    view: ApplicationView<AuthenticatePasswordAction>
    store: Readonly<{
        loginID: BoardValueStore
        password: BoardValueStore
    }>
}> {
    const currentURL = new URL("https://example.com/index.html")

    const ticketRepository = standard_ticketRepository()

    const view = toApplicationView(
        initAuthenticatePasswordAction(
            {
                continuousRenewInterval: { interval_millisecond: 128 },
                ticketExpire: { expire_millisecond: 500 },
                takeLongtimeThreshold: { delay_millisecond: 32 },
                secureServerURL: mockSecureServerURL("https://secure.example.com"),
            },
            {
                ticketRepository,
                renewRemote,
                authenticateRemote,
                clock,
            },
            {
                ...mockGetScriptPathShell(currentURL),
            },
        ),
    )

    const store = {
        loginID: mockBoardValueStore(),
        password: mockBoardValueStore(),
    }

    view.resource.loginID.input.connector.connect(store.loginID)
    view.resource.password.input.connector.connect(store.password)

    return { view, store }
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        roles: ["role"],
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
        value: convertAuthRemote(clock, { roles: ["role"] }),
    }
}

function standard_renew(clock: Clock, clockPubSub: ClockPubSub): RenewAuthTicketRemote {
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
            value: convertAuthRemote(clock, { roles: ["role"] }),
        }
    }
}