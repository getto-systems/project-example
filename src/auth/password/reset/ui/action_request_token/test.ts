import { setupActionTestRunner } from "../../../../../../ui/vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/action_input/mock"
import { mockRemotePod } from "../../../../../z_details/_ui/remote/mock"

import { initRequestResetTokenView } from "./impl"
import { initRequestResetTokenCoreMaterial, initRequestResetTokenCoreAction } from "./core/impl"
import { initRequestResetTokenFormAction } from "./form/impl"

import { resetSessionIDRemoteConverter } from "../converter"

import { RequestResetTokenRemotePod, RequestResetTokenResult } from "../request_token/infra"

import { RequestResetTokenView } from "./resource"

const VALID_LOGIN = { loginID: "login-id" } as const

describe("RequestResetToken", () => {
    test("submit valid login-id", async () => {
        const { view } = standard()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            action.form.loginID.board.input.set(markBoardValue(VALID_LOGIN.loginID))
            return action.core.submit(action.form.validate.get())
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-request-token" },
                { type: "succeed-to-request-token", sessionID: "session-id" },
            ])
        })
    })

    test("submit valid login-id; with take longtime", async () => {
        // wait for take longtime timeout
        const { view } = takeLongtime()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner(action.core.subscriber)

        await runner(() => {
            action.form.loginID.board.input.set(markBoardValue(VALID_LOGIN.loginID))
            return action.core.submit(action.form.validate.get())
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-request-token" },
                { type: "take-longtime-to-request-token" },
                { type: "succeed-to-request-token", sessionID: "session-id" },
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
        const { view } = standard()
        const resource = view.resource.requestToken

        resource.form.loginID.board.input.set(markBoardValue(VALID_LOGIN.loginID))
        resource.form.clear()

        expect(resource.form.loginID.board.input.get()).toEqual("")
    })

    test("terminate", async () => {
        const { view } = standard()
        const action = view.resource.requestToken

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.core.subscriber.subscribe(handler)
                action.form.validate.subscriber.subscribe(handler)
                action.form.loginID.validate.subscriber.subscribe(handler)
                action.form.loginID.board.input.subscribeInputEvent(() => handler("input"))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            view.terminate()
            action.form.loginID.board.input.set(markBoardValue("login-id"))
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const view = initView(standard_requestToken())

    return { view }
}
function takeLongtime() {
    const view = initView(takeLongtime_requestToken())

    return { view }
}

function initView(requestToken: RequestResetTokenRemotePod): RequestResetTokenView {
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

    view.resource.requestToken.form.loginID.board.input.storeLinker.link(mockBoardValueStore())

    return view
}

function standard_requestToken(): RequestResetTokenRemotePod {
    return mockRemotePod(simulateRequestToken, { wait_millisecond: 0 })
}
function takeLongtime_requestToken(): RequestResetTokenRemotePod {
    return mockRemotePod(simulateRequestToken, { wait_millisecond: 64 })
}
function simulateRequestToken(): RequestResetTokenResult {
    return { success: true, value: resetSessionIDRemoteConverter("session-id") }
}
