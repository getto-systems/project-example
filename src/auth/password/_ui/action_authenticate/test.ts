import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_details/_ui/timer/helper"

import { ClockPubSub, mockClock, mockClockPubSub } from "../../../../z_details/_ui/clock/mock"

import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/action_input/mock"
import { mockGetScriptPathDetecter } from "../../../_ui/common/secure/get_script_path/mock"

import { initAuthenticatePasswordView } from "./impl"
import {
    initAuthenticatePasswordCoreAction,
    initAuthenticatePasswordCoreMaterial,
} from "./core/impl"
import { initAuthenticatePasswordFormAction } from "./form/impl"

import { Clock } from "../../../../z_details/_ui/clock/infra"
import { AuthenticatePasswordRemote, AuthenticatePasswordRemoteResult } from "../authenticate/infra"
import {
    AuthnRepository,
    AuthzRepository,
    RenewAuthTicketRemote,
} from "../../../auth_ticket/_ui/kernel/infra"

import { AuthenticatePasswordView } from "./resource"

import {
    authzRepositoryConverter,
    convertAuthRemote,
} from "../../../auth_ticket/_ui/kernel/convert"

import { LoadScriptError } from "../../../_ui/common/secure/get_script_path/data"
import {
    mockAuthnRepository,
    mockAuthzRepository,
} from "../../../auth_ticket/_ui/kernel/init/repository/mock"

// テスト開始時刻
const START_AT = new Date("2020-01-01 10:00:00")

// renew 設定時刻 : succeed-to-start-continuous-renew でこの時刻に移行
const CONTINUOUS_RENEW_START_AT = new Date("2020-01-01 10:00:01")

// renew ごとに次の時刻に移行
const CONTINUOUS_RENEW_AT = [new Date("2020-01-01 10:01:00"), new Date("2020-01-01 11:00:00")]

const VALID_LOGIN = { loginID: "login-id", password: "password" } as const

describe("AuthenticatePassword", () => {
    test("submit valid login-id and password", async () => {
        const { clock, view } = standard()
        const resource = view.resource.authenticate

        resource.core.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(async () => {
            resource.form.loginID.board.input.set(markBoardValue(VALID_LOGIN.loginID))
            resource.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))

            return resource.core.submit(resource.form.validate.get())
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
        const { clock, view } = takeLongtime_elements()
        const resource = view.resource.authenticate

        resource.core.subscriber.subscribe((state) => {
            switch (state.type) {
                case "try-to-load":
                    clock.update(CONTINUOUS_RENEW_START_AT)
                    break
            }
        })

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => {
            resource.form.loginID.board.input.set(markBoardValue(VALID_LOGIN.loginID))
            resource.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))

            return resource.core.submit(resource.form.validate.get())
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
        const resource = view.resource.authenticate

        const runner = setupActionTestRunner(resource.core.subscriber)

        await runner(() => resource.core.submit(resource.form.validate.get())).then((stack) => {
            expect(stack).toEqual([{ type: "failed-to-login", err: { type: "validation-error" } }])
        })
    })

    test("clear", () => {
        const { view } = standard()
        const resource = view.resource.authenticate

        resource.form.loginID.board.input.set(markBoardValue(VALID_LOGIN.loginID))
        resource.form.password.board.input.set(markBoardValue(VALID_LOGIN.password))
        resource.form.clear()

        expect(resource.form.loginID.board.input.get()).toEqual("")
        expect(resource.form.password.board.input.get()).toEqual("")
    })

    test("load error", async () => {
        const { view } = standard()
        const resource = view.resource.authenticate

        const runner = setupActionTestRunner(resource.core.subscriber)

        const err: LoadScriptError = { type: "infra-error", err: "load error" }

        await runner(() => resource.core.loadError(err)).then((stack) => {
            expect(stack).toEqual([{ type: "load-error", err }])
        })
    })

    test("terminate", async () => {
        const { view } = standard()
        const resource = view.resource.authenticate

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                resource.core.subscriber.subscribe(handler)
                resource.form.validate.subscriber.subscribe(handler)
                resource.form.loginID.validate.subscriber.subscribe(handler)
                resource.form.password.validate.subscriber.subscribe(handler)
                resource.form.loginID.board.input.subscribeInputEvent(() => handler("input"))
                resource.form.password.board.input.subscribeInputEvent(() => handler("input"))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            view.terminate()
            resource.form.loginID.board.input.set(markBoardValue("login-id"))
            resource.form.password.board.input.set(markBoardValue("password"))
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

    return { clock: clockPubSub, view }
}
function takeLongtime_elements() {
    const clockPubSub = mockClockPubSub()
    const clock = mockClock(START_AT, clockPubSub)
    const view = initView(
        takeLongtime_authenticate(clock),
        standard_renew(clock, clockPubSub),
        clock,
    )

    return { clock: clockPubSub, view }
}

function initView(
    authenticate: AuthenticatePasswordRemote,
    renew: RenewAuthTicketRemote,
    clock: Clock,
): AuthenticatePasswordView {
    const currentURL = new URL("https://example.com/index.html")

    const authn = standard_authn()
    const authz = standard_authz()

    const getScriptPathDetecter = mockGetScriptPathDetecter(currentURL)

    const view = initAuthenticatePasswordView({
        core: initAuthenticatePasswordCoreAction(
            initAuthenticatePasswordCoreMaterial(
                {
                    startContinuousRenew: {
                        authn,
                        authz,
                        renew,
                        config: {
                            interval: { interval_millisecond: 128 },
                            authnExpire: { expire_millisecond: 500 },
                        },
                        clock,
                    },
                    getSecureScriptPath: {
                        config: {
                            secureServerURL: "https://secure.example.com",
                        },
                    },
                    authenticate: {
                        authenticate,
                        config: {
                            takeLongtimeThreshold: { delay_millisecond: 32 },
                        },
                    },
                },
                getScriptPathDetecter,
            ),
        ),

        form: initAuthenticatePasswordFormAction(),
    })

    view.resource.authenticate.form.loginID.board.input.storeLinker.link(mockBoardValueStore())
    view.resource.authenticate.form.password.board.input.storeLinker.link(mockBoardValueStore())

    return view
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
