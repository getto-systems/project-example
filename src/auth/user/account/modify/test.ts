import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../common/util/timer/helper"
import {
    mockSingleBoardStore,
    mockMultipleBoardStore,
} from "../../../../common/util/board/input/test_helper"

import { restoreLoginId } from "../../login_id/kernel/convert"
import { restoreAuthUserField } from "../kernel/convert"

import { initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import {
    LoadState,
    loadState_loaded,
    loadState_loading,
    mapLoadState,
} from "../../../../common/util/load/data"
import { initLoadableListAtomUpdater } from "../../../../common/util/list/action"
import { ModifyAuthUserAccountAction, initModifyAuthUserAccountAction } from "./action"

import { ModifyAuthUserAccountRemote } from "./infra"
import { SingleBoardStore, MultipleBoardStore } from "../../../../common/util/board/input/infra"

import { AuthUserAccount } from "../kernel/data"

const VALID_INFO = {
    memo: "memo",
    granted: ["auth-user"],
} as const

test("submit valid info", async () => {
    const { modify, store } = standard()

    const result = observeAtom(modify.state)

    store.memo.set(VALID_INFO.memo)
    store.granted.set(VALID_INFO.granted)

    await modify.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: "user-id",
                granted: ["auth-user"],
                resetTokenDestination: { type: "none" },
                memo: "memo",
            },
        },
        { type: "initial" },
    ])
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { modify, store } = takeLongtime_elements()

    const result = observeAtom(modify.state)

    store.memo.set(VALID_INFO.memo)
    store.granted.set(VALID_INFO.granted)

    await modify.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: "user-id",
                granted: ["auth-user"],
                resetTokenDestination: { type: "none" },
                memo: "memo",
            },
        },
        { type: "initial" },
    ])
})

test("reset", () => {
    const { modify, store } = standard()

    store.memo.set(VALID_INFO.memo)
    store.granted.set(VALID_INFO.granted)

    modify.reset()

    expect(store.memo.get()).toEqual("initial-memo")
    expect(store.granted.get()).toEqual([])
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
        memo: SingleBoardStore
        granted: MultipleBoardStore
    }>
}> {
    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })
    const data = mapAtom(list.state, (list) => mapLoadState(list, (list) => list[0]))
    const modify = initModifyAuthUserAccountAction(data, initLoadableListAtomUpdater(list), {
        infra: {
            modifyUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    list.post(
        loadState_loaded([
            {
                loginId: restoreLoginId("user-id"),
                granted: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("initial-memo"),
            },
        ]),
    )

    return {
        modify,
        store: {
            memo: mockSingleBoardStore(modify.memo.input),
            granted: mockMultipleBoardStore(modify.granted.input),
        },
    }
}

function standard_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
