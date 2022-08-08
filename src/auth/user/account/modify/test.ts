import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"
import {
    mockBoardValueStore,
    mockMultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ModifyAuthUserAccountAction, initModifyAuthUserAccountAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"
import { restoreAuthUserField } from "../kernel/convert"

import { ModifyAuthUserAccountRemote } from "./infra"
import {
    BoardValueStore,
    MultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/infra"

const VALID_INFO = {
    memo: "memo",
    grantedRoles: ["auth-user"],
} as const

test("submit valid info", async () => {
    const { modify, store } = standard()

    expect(
        await observeApplicationState(modify.state, async () => {
            store.memo.set(VALID_INFO.memo)
            store.grantedRoles.set(VALID_INFO.grantedRoles)

            return modify.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            entry: { loginId: "user-id", grantedRoles: ["auth-user"], memo: "memo" },
        },
        { type: "initial" },
    ])
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { modify, store } = takeLongtime_elements()

    expect(
        await observeApplicationState(modify.state, async () => {
            store.memo.set(VALID_INFO.memo)
            store.grantedRoles.set(VALID_INFO.grantedRoles)

            return modify.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            entry: { loginId: "user-id", grantedRoles: ["auth-user"], memo: "memo" },
        },
        { type: "initial" },
    ])
})

test("reset", () => {
    const { modify, store } = standard()

    store.memo.set(VALID_INFO.memo)
    store.grantedRoles.set(VALID_INFO.grantedRoles)

    modify.reset()

    expect(store.memo.get()).toEqual("initial-memo")
    expect(store.grantedRoles.get()).toEqual([])
})

function standard() {
    return initResource(standard_modifyUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_modifyUserRemote())
}

function initResource(modifyUserRemote: ModifyAuthUserAccountRemote): Readonly<{
    modify: ModifyAuthUserAccountAction
    store: Readonly<{
        memo: BoardValueStore
        grantedRoles: MultipleBoardValueStore
    }>
}> {
    const modify = initModifyAuthUserAccountAction({
        infra: {
            modifyUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    modify.handler.focus({
        loginId: restoreLoginId("user-id"),
        grantedRoles: [],
        memo: restoreAuthUserField("initial-memo"),
    })

    return {
        modify: modify.action,
        store: {
            memo: mockBoardValueStore(modify.action.memo.input),
            grantedRoles: mockMultipleBoardValueStore(modify.action.grantedRoles.input),
        },
    }
}

function standard_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
