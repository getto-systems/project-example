import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"
import { toApplicationView } from "../../../../../z_vendor/getto-application/action/helper"
import { ticker } from "../../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { RequestResetTokenRemote, RequestResetTokenRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../../z_vendor/getto-application/board/input/infra"
import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { initRequestResetTokenAction, RequestResetTokenAction } from "./action"

const VALID_LOGIN = { loginId: "login-id" } as const

test("submit valid login-id", async () => {
    const { view, store } = standard()
    const action = view.resource

    const runner = setupActionTestRunner(action.subscriber)

    await runner(() => {
        store.loginId.set(markBoardValue(VALID_LOGIN.loginId))
        return action.submit()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try" }, { type: "success" }])
    })
})

test("submit valid login-id; with take longtime", async () => {
    // wait for take longtime timeout
    const { view, store } = takeLongtime()
    const action = view.resource

    const runner = setupActionTestRunner(action.subscriber)

    await runner(() => {
        store.loginId.set(markBoardValue(VALID_LOGIN.loginId))
        return action.submit()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try" }, { type: "take-longtime" }, { type: "success" }])
    })
})

test("submit without fields", async () => {
    const { view } = standard()
    const action = view.resource

    const runner = setupActionTestRunner(action.subscriber)

    await runner(() => action.submit()).then((stack) => {
        expect(stack).toEqual([{ type: "failed", err: { type: "validation-error" } }])
    })
})

test("clear", () => {
    const { view, store } = standard()
    const resource = view.resource

    store.loginId.set(markBoardValue(VALID_LOGIN.loginId))
    resource.clear()

    expect(store.loginId.get()).toEqual("")
})

test("terminate", async () => {
    const { view } = standard()
    const action = view.resource

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            action.subscriber.subscribe(handler)
            action.validate.subscriber.subscribe(handler)
            action.loginId.validate.subscriber.subscribe(handler)
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

function standard() {
    return initView(standard_requestToken())
}
function takeLongtime() {
    return initView(takeLongtime_requestToken())
}

function initView(requestTokenRemote: RequestResetTokenRemote): Readonly<{
    view: ApplicationView<RequestResetTokenAction>
    store: Readonly<{
        loginId: BoardValueStore
    }>
}> {
    const view = toApplicationView(
        initRequestResetTokenAction({
            infra: {
                requestTokenRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    )

    const store = {
        loginId: mockBoardValueStore(),
    }
    view.resource.loginId.input.connector.connect(store.loginId)

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
