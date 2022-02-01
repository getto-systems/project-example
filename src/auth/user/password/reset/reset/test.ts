import { setupActionTestRunner } from "../../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../z_lib/ui/timer/helper"
import { toApplicationView } from "../../../../../../ui/vendor/getto-application/action/helper"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/init/mock"
import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../../z_lib/ui/clock/mock"
import { mockSecureServerURL } from "../../../../sign/get_script_path/init/mock"
import { mockResetPasswordShell } from "./init/mock"

import { Clock } from "../../../../../z_lib/ui/clock/infra"
import { ResetPasswordRemote, ResetPasswordRemoteResult } from "./infra"
import {
    AuthTicketRepository,
    AuthTicketRepositoryValue,
} from "../../../../ticket/kernel/infra"
import { CheckAuthTicketRemote } from "../../../../ticket/check/infra"
import { BoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/infra"

import { authTicketRepositoryConverter, convertAuthRemote } from "../../../../ticket/kernel/convert"
import { initMemoryDB } from "../../../../../z_lib/ui/repository/init/memory"
import { convertDB } from "../../../../../z_lib/ui/repository/init/convert"
import { initResetPasswordAction, ResetPasswordAction } from "./action"
import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"

// テスト開始時刻
const START_AT = new Date("2020-01-01 10:00:00")

// renew 設定時刻 : succeed-to-start-continuous-renew でこの時刻に移行
const CONTINUOUS_RENEW_START_AT = new Date("2020-01-01 10:00:01")

// renew ごとに次の時刻に移行
const CONTINUOUS_RENEW_AT = [new Date("2020-01-01 10:01:00"), new Date("2020-01-01 11:00:00")]

const VALID_LOGIN = { loginID: "login-id", password: "password" } as const

describe("RegisterPassword", () => {
    test("submit valid login-id and password", async () => {
        const { clock, view, store } = standard()
        const action = view.resource

        action.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            store.password.set(markBoardValue(VALID_LOGIN.password))
            return action.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-reset" },
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

    test("submit valid login-id and password; with take longtime", async () => {
        // wait for take longtime timeout
        const { clock, view, store } = takeLongtime()
        const action = view.resource

        action.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            store.password.set(markBoardValue(VALID_LOGIN.password))
            return action.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-reset" },
                { type: "take-longtime-to-reset" },
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

    test("submit without fields", async () => {
        const { view } = standard()
        const action = view.resource

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.submit()).then((stack) => {
            expect(stack).toEqual([{ type: "failed-to-reset", err: { type: "validation-error" } }])
        })
    })

    test("submit without resetToken", async () => {
        const { view, store } = emptyResetToken()
        const action = view.resource

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            store.password.set(markBoardValue(VALID_LOGIN.password))
            return action.submit()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "failed-to-reset", err: { type: "empty-reset-token" } }])
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
        const action = view.resource

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.loadError({ type: "infra-error", err: "load error" })).then(
            (stack) => {
                expect(stack).toEqual([
                    { type: "load-error", err: { type: "infra-error", err: "load error" } },
                ])
            },
        )
    })

    test("terminate", async () => {
        const { view } = standard()
        const action = view.resource

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.subscriber.subscribe(handler)
                action.validate.subscriber.subscribe(handler)
                action.loginID.validate.subscriber.subscribe(handler)
                action.password.validate.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            view.terminate()
            action.submit()
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
        standard_URL(),
        standard_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...view }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        standard_URL(),
        takeLongtime_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...view }
}
function emptyResetToken() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    return initView(
        emptyResetToken_URL(),
        standard_resetRemote(clock),
        standard_renewRemote(clock, clockPubSub),
        clock,
    )
}

function initView(
    currentURL: URL,
    resetRemote: ResetPasswordRemote,
    renewRemote: CheckAuthTicketRemote,
    clock: Clock,
): Readonly<{
    view: ApplicationView<ResetPasswordAction>
    store: Readonly<{
        loginID: BoardValueStore
        password: BoardValueStore
    }>
}> {
    const ticketRepository = standard_ticketRepository()

    const view = toApplicationView(
        initResetPasswordAction({
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
                takeLongtimeThreshold: { delay_millisecond: 32 },
                secureServerURL: mockSecureServerURL("https://secure.example.com"),
            },
        }),
    )

    const store = {
        loginID: mockBoardValueStore(),
        password: mockBoardValueStore(),
    }
    view.resource.loginID.input.connector.connect(store.loginID)
    view.resource.password.input.connector.connect(store.password)

    return { view, store }
}

function standard_URL(): URL {
    return new URL("https://example.com/index.html?-password-reset-token=reset-token")
}
function emptyResetToken_URL(): URL {
    return new URL("https://example.com/index.html")
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        roles: ["role"],
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
        value: convertAuthRemote(clock, { roles: ["role"] }),
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
            value: convertAuthRemote(clock, { roles: ["role"] }),
        }
    }
}
