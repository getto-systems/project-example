import { test, expect } from "vitest"
import { observeAtom } from "../../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../../common/util/timer/helper"

import { mockSingleBoardStore } from "../../../../../common/util/board/input/test_helper"

import { RequestResetTokenRemote, RequestResetTokenRemoteResult } from "./infra"
import { SingleBoardStore } from "../../../../../common/util/board/input/infra"
import { initRequestResetTokenAction, RequestResetTokenAction } from "./action"

const VALID_LOGIN = { loginId: "login-id" } as const

test("submit valid login-id", async () => {
    const { action, store } = standard()

    const result = observeAtom(action.state)

    store.loginId.set(VALID_LOGIN.loginId)

    await action.submit()

    expect(result()).toEqual([{ type: "try", hasTakenLongtime: false }, { type: "success" }])
})

test("submit valid login-id; with take longtime", async () => {
    // wait for take longtime timeout
    const { action, store } = takeLongtime()

    const result = observeAtom(action.state)

    store.loginId.set(VALID_LOGIN.loginId)

    await action.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        { type: "success" },
    ])
})

test("submit without fields", async () => {
    const { action } = standard()

    const result = observeAtom(action.state)

    await action.submit()

    expect(result()).toEqual([])
})

test("edit", () => {
    const { action, store } = standard()

    store.loginId.set(VALID_LOGIN.loginId)

    action.editable.open()

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
        loginId: SingleBoardStore
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
        loginId: mockSingleBoardStore(action.loginId.input),
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
