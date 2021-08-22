import { setupActionTestRunner } from "../../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../z_details/_ui/timer/helper"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/init/mock"

import { initRequestResetTokenView } from "./impl"
import { initRequestResetTokenCoreMaterial, initRequestResetTokenCoreAction } from "./core/impl"
import { initRequestResetTokenFormAction } from "./form/impl"

import { RequestResetTokenRemote, RequestResetTokenRemoteResult } from "../request_token/infra"
import { BoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/infra"

import { RequestResetTokenView } from "./resource"

const VALID_LOGIN = { loginID: "login-id" } as const

describe("RequestResetToken", () => {
    test("submit valid login-id", async () => {
        const { view, store } = standard()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            return action.core.submit(action.form.validate.get())
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-request-token" },
                { type: "succeed-to-request-token" },
            ])
        })
    })

    test("submit valid login-id; with take longtime", async () => {
        // wait for take longtime timeout
        const { view, store } = takeLongtime()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            return action.core.submit(action.form.validate.get())
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-request-token" },
                { type: "take-longtime-to-request-token" },
                { type: "succeed-to-request-token" },
            ])
        })
    })

    test("submit without fields", async () => {
        const { view } = standard()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => action.core.submit(action.form.validate.get())).then((stack) => {
            expect(stack).toEqual([
                { type: "failed-to-request-token", err: { type: "validation-error" } },
            ])
        })
    })

    test("clear", () => {
        const { view, store } = standard()
        const resource = view.resource.requestToken

        store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
        resource.form.clear()

        expect(store.loginID.get()).toEqual("")
    })

    test("terminate", async () => {
        const { view } = standard()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.core.subscriber.subscribe(handler)
                action.form.validate.subscriber.subscribe(handler)
                action.form.loginID.validate.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            view.terminate()
            action.form.loginID.validate.check()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    return initView(standard_requestToken())
}
function takeLongtime() {
    return initView(takeLongtime_requestToken())
}

function initView(requestToken: RequestResetTokenRemote): Readonly<{
    view: RequestResetTokenView
    store: Readonly<{
        loginID: BoardValueStore
    }>
}> {
    const view = initRequestResetTokenView({
        core: initRequestResetTokenCoreAction(
            initRequestResetTokenCoreMaterial({
                requestToken,
                config: {
                    takeLongtimeThreshold: { delay_millisecond: 32 },
                },
            }),
        ),

        form: initRequestResetTokenFormAction(),
    })

    const store = {
        loginID: mockBoardValueStore(),
    }
    view.resource.requestToken.form.loginID.input.connector.connect(store.loginID)

    return { view, store }
}

function standard_requestToken(): RequestResetTokenRemote {
    return async () => standard_requestResetTokenRemoteResult()
}
function takeLongtime_requestToken(): RequestResetTokenRemote {
    return async () =>
        ticker({ wait_millisecond: 64 }, () => standard_requestResetTokenRemoteResult())
}
function standard_requestResetTokenRemoteResult(): RequestResetTokenRemoteResult {
    return { success: true, value: true }
}
