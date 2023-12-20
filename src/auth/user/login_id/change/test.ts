import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { mockSingleBoardStore } from "../../../../common/util/board/input/test_helper"
import { ticker } from "../../../../common/util/timer/helper"

import { restoreLoginId } from "../kernel/convert"
import { restoreAuthUserField } from "../../account/kernel/convert"

import { initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import {
    LoadState,
    loadState_loaded,
    loadState_loading,
    mapLoadState,
} from "../../../../common/util/load/data"
import { initLoadableListAtomUpdater } from "../../../../common/util/list/action"
import { OverwriteLoginIdAction, initOverwriteLoginIdAction } from "./action"

import { OverwriteLoginIdRemote, ChangePasswordRemoteResult } from "./infra"
import { SingleBoardStore } from "../../../../common/util/board/input/infra"

import { AuthUserAccount } from "../../account/kernel/data"

const VALID_LOGIN_ID = { newLoginId: "new-login-id" } as const

test("submit valid new-login-id", async () => {
    const { overwrite, store } = standard()

    const result = observeAtom(overwrite.state)

    store.newLoginId.set(VALID_LOGIN_ID.newLoginId)
    await overwrite.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: "new-login-id",
                granted: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("initial-memo"),
            },
        },
        { type: "initial" },
    ])
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { overwrite, store } = takeLongtime_elements()

    const result = observeAtom(overwrite.state)

    store.newLoginId.set(VALID_LOGIN_ID.newLoginId)
    await overwrite.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: "new-login-id",
                granted: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("initial-memo"),
            },
        },
        { type: "initial" },
    ])
})

test("submit without fields", async () => {
    const { overwrite } = standard()

    const result = observeAtom(overwrite.state)

    await overwrite.submit()

    expect(result()).toEqual([])
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
        newLoginId: SingleBoardStore
    }>
}> {
    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })
    const data = mapAtom(list.state, (list) => mapLoadState(list, (list) => list[0]))
    const overwrite = initOverwriteLoginIdAction(data, initLoadableListAtomUpdater(list), {
        infra: {
            overwriteLoginIdRemote,
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
        overwrite,
        store: {
            newLoginId: mockSingleBoardStore(overwrite.newLoginId.input),
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
