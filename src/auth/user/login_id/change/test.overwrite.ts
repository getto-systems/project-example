import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverwriteLoginIdAction, initOverwriteLoginIdAction } from "./action"

import { restoreLoginId } from "../input/convert"

import { OverwriteLoginIdRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

const VALID_LOGIN_ID = { newLoginId: "new-login-id" } as const

test("submit valid new-login-id", async () => {
    const { overwrite, store } = standard()

    const runner = setupActionTestRunner(overwrite.state)

    await runner(async () => {
        store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

        return overwrite.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success", entry: { loginId: "new-login-id" } },
            { type: "initial" },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { overwrite, store } = takeLongtime_elements()

    const runner = setupActionTestRunner(overwrite.state)

    await runner(() => {
        store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

        return overwrite.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success", entry: { loginId: "new-login-id" } },
            { type: "initial" },
        ])
    })
})

test("submit without fields", async () => {
    const { overwrite } = standard()

    const runner = setupActionTestRunner(overwrite.state)

    await runner(() => overwrite.submit()).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("reset", () => {
    const { overwrite, store } = standard()

    store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

    overwrite.reset()

    expect(store.newLoginId.get()).toEqual("")
})

function standard() {
    return initResource(standard_overwriteRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_overwriteRemote())
}

function initResource(overwriteLoginIdRemote: OverwriteLoginIdRemote): Readonly<{
    overwrite: OverwriteLoginIdAction
    store: Readonly<{
        newLoginId: BoardValueStore
    }>
}> {
    const overwrite = initOverwriteLoginIdAction({
        infra: {
            overwriteLoginIdRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    overwrite.handler.focus({
        loginId: restoreLoginId("user-id"),
    })

    return {
        overwrite: overwrite.action,
        store: {
            newLoginId: mockBoardValueStore(overwrite.action.newLoginId.input),
        },
    }
}

function standard_overwriteRemote(): OverwriteLoginIdRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_overwriteRemote(): OverwriteLoginIdRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
