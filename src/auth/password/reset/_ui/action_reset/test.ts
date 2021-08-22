import { setupActionTestRunner } from "../../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../z_details/_ui/timer/helper"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/action_input/mock"
import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../../z_details/_ui/clock/mock"

import { mockGetScriptPathDetecter } from "../../../../_ui/common/secure/get_script_path/mock"
import { mockResetPasswordLocationDetecter } from "../reset/mock"

import { initResetPasswordView } from "./impl"
import { initResetPasswordCoreAction, initResetPasswordCoreMaterial } from "./core/impl"
import { initResetPasswordFormAction } from "./form/impl"

import { Clock } from "../../../../../z_details/_ui/clock/infra"
import { ResetPasswordRemote, ResetPasswordRemoteResult } from "../reset/infra"
import {
    AuthnRepository,
    AuthzRepository,
    RenewAuthTicketRemote,
} from "../../../../auth_ticket/_ui/kernel/infra"

import { ResetPasswordView } from "./resource"

import {
    authzRepositoryConverter,
    convertAuthRemote,
} from "../../../../auth_ticket/_ui/kernel/convert"
import {
    mockAuthnRepository,
    mockAuthzRepository,
} from "../../../../auth_ticket/_ui/kernel/init/repository/mock"
import { BoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/infra"

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
        const action = view.resource.reset

        action.core.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            action.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))
            return action.core.submit(action.form.validate.get())
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
        const action = view.resource.reset

        action.core.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            action.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))
            return action.core.submit(action.form.validate.get())
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
        const action = view.resource.reset

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => action.core.submit(action.form.validate.get())).then((stack) => {
            expect(stack).toEqual([{ type: "failed-to-reset", err: { type: "validation-error" } }])
        })
    })

    test("submit without resetToken", async () => {
        const { view, store } = emptyResetToken()
        const action = view.resource.reset

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            action.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))
            return action.core.submit(action.form.validate.get())
        }).then((stack) => {
            expect(stack).toEqual([{ type: "failed-to-reset", err: { type: "empty-reset-token" } }])
        })
    })

    test("clear", () => {
        const { view, store } = standard()
        const resource = view.resource.reset

        store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
        resource.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))
        resource.form.clear()

        expect(store.loginID.get()).toEqual("")
        expect(resource.form.password.board.input.get()).toEqual("")
    })

    test("load error", async () => {
        const { view } = standard()
        const action = view.resource.reset

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => action.core.loadError({ type: "infra-error", err: "load error" })).then(
            (stack) => {
                expect(stack).toEqual([
                    { type: "load-error", err: { type: "infra-error", err: "load error" } },
                ])
            },
        )
    })

    test("terminate", async () => {
        const { view } = standard()
        const action = view.resource.reset

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.core.subscriber.subscribe(handler)
                action.form.validate.subscriber.subscribe(handler)
                action.form.loginID.validate.subscriber.subscribe(handler)
                action.form.password.validate.subscriber.subscribe(handler)
                action.form.password.board.input.subscribeInputEvent(() => handler("input"))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            view.terminate()
            action.form.password.board.input.set(markBoardValue("password"))
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
        standard_reset(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...view }
}
function takeLongtime() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        standard_URL(),
        takeLongtime_reset(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, ...view }
}
function emptyResetToken() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    return initView(
        emptyResetToken_URL(),
        standard_reset(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )
}

function initView(
    currentURL: URL,
    reset: ResetPasswordRemote,
    renew: RenewAuthTicketRemote,
    clock: Clock,
): Readonly<{
    view: ResetPasswordView
    store: Readonly<{
        loginID: BoardValueStore
    }>
}> {
    const authn = standard_authn()
    const authz = standard_authz()

    const detecter = {
        getSecureScriptPath: mockGetScriptPathDetecter(currentURL),
        reset: mockResetPasswordLocationDetecter(currentURL),
    }

    const view = initResetPasswordView({
        core: initResetPasswordCoreAction(
            initResetPasswordCoreMaterial(
                {
                    startContinuousRenew: {
                        authn: authn,
                        authz,
                        renew,
                        config: {
                            interval: { interval_millisecond: 64 },
                            authnExpire: { expire_millisecond: 500 },
                        },
                        clock,
                    },
                    getSecureScriptPath: {
                        config: {
                            secureServerURL: "https://secure.example.com",
                        },
                    },
                    reset: {
                        reset,
                        config: {
                            takeLongtimeThreshold: { delay_millisecond: 32 },
                        },
                    },
                },
                detecter,
            ),
        ),

        form: initResetPasswordFormAction(),
    })

    const store = {
        loginID: mockBoardValueStore(),
    }
    view.resource.reset.form.loginID.input.connector.connect(store.loginID)
    view.resource.reset.form.password.board.input.storeLinker.link(mockBoardValueStore())

    return { view, store }
}

function standard_URL(): URL {
    return new URL("https://example.com/index.html?-password-reset-token=reset-token")
}
function emptyResetToken_URL(): URL {
    return new URL("https://example.com/index.html")
}

function standard_authn(): AuthnRepository {
    return mockAuthnRepository()
}
function standard_authz(): AuthzRepository {
    const result = authzRepositoryConverter.fromRepository({
        roles: ["role"],
    })
    if (!result.valid) {
        throw new Error("invalid authz")
    }

    const repository = mockAuthzRepository()
    repository.set(result.value)
    return repository
}

function standard_reset(clock: Clock): ResetPasswordRemote {
    return async () => standard_resetPasswordRemoteResult(clock)
}
function takeLongtime_reset(clock: Clock): ResetPasswordRemote {
    return async () =>
        ticker({ wait_millisecond: 64 }, () => standard_resetPasswordRemoteResult(clock))
}
function standard_resetPasswordRemoteResult(clock: Clock): ResetPasswordRemoteResult {
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
