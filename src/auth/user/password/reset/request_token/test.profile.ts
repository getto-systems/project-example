import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/init/mock"

import { RequestResetTokenRemote, RequestResetTokenRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../../z_vendor/getto-application/board/input/infra"
import { initRequestResetTokenProfileAction, RequestResetTokenProfileAction } from "./action"

const VALID_LOGIN = { loginID: "login-id" } as const

describe("RequestResetTokenProfile", () => {
    test("submit valid login-id", async () => {
        const { resource, store } = standard()
        const action = resource.requestToken

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            return action.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-request-token" },
                { type: "succeed-to-request-token" },
            ])
        })
    })

    test("submit valid login-id; with take longtime", async () => {
        // wait for take longtime timeout
        const { resource, store } = takeLongtime()
        const action = resource.requestToken

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
            return action.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-request-token" },
                { type: "take-longtime-to-request-token" },
                { type: "succeed-to-request-token" },
            ])
        })
    })

    test("submit without fields", async () => {
        const { resource } = standard()
        const action = resource.requestToken

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.submit()).then((stack) => {
            expect(stack).toEqual([
                { type: "failed-to-request-token", err: { type: "validation-error" } },
            ])
        })
    })

    test("clear", () => {
        const { resource, store } = standard()
        const action = resource.requestToken

        store.loginID.set(markBoardValue(VALID_LOGIN.loginID))
        action.clear()

        expect(store.loginID.get()).toEqual("")
    })

    test("open; close", async () => {
        const { resource } = standard()
        const action = resource.requestToken

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            action.open()
            action.close()
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "input-login-id" }, { type: "initial-request-token" }])
        })
    })

    test("terminate", async () => {
        const { resource } = standard()
        const action = resource.requestToken

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.subscriber.subscribe(handler)
                action.validate.subscriber.subscribe(handler)
                action.loginID.validate.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.terminate()
            action.submit()
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

function initView(requestTokenRemote: RequestResetTokenRemote): Readonly<{
    resource: Readonly<{
        requestToken: RequestResetTokenProfileAction
    }>
    store: Readonly<{
        loginID: BoardValueStore
    }>
}> {
    const resource = {
        requestToken: initRequestResetTokenProfileAction({
            infra: {
                requestTokenRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        loginID: mockBoardValueStore(),
    }
    resource.requestToken.loginID.input.connector.connect(store.loginID)

    return { resource, store }
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
