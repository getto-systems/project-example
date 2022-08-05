import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { RequestResetTokenRemote, RequestResetTokenRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../../z_vendor/getto-application/board/input/infra"
import { initRequestResetTokenAction, RequestResetTokenAction } from "./action"

const VALID_LOGIN = { loginId: "login-id" } as const

test("submit valid login-id", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.state)

    await runner(() => {
        store.loginId.set(VALID_LOGIN.loginId)
        return action.submit(() => null)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try", hasTakenLongtime: false }, { type: "success" }])
    })
})

test("submit valid login-id; with take longtime", async () => {
    // wait for take longtime timeout
    const { action, store } = takeLongtime()

    const runner = setupActionTestRunner(action.state)

    await runner(() => {
        store.loginId.set(VALID_LOGIN.loginId)
        return action.submit(() => null)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success" },
        ])
    })
})

test("submit without fields", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.state)

    await runner(() => action.submit(() => null)).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("edit", () => {
    const { action, store } = standard()

    store.loginId.set(VALID_LOGIN.loginId)

    action.edit()

    expect(store.loginId.get()).toEqual("")
})

function standard() {
    return initResource(standard_requestToken())
}
function takeLongtime() {
    return initResource(takeLongtime_requestToken())
}

function initResource(requestTokenRemote: RequestResetTokenRemote): Readonly<{
    action: RequestResetTokenAction
    store: Readonly<{
        loginId: BoardValueStore
    }>
}> {
    const action = initRequestResetTokenAction({
        infra: {
            requestTokenRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
        },
    })

    const store = {
        loginId: mockBoardValueStore(action.loginId.input),
    }

    return { action, store }
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
