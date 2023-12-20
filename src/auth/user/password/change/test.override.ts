import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../common/util/timer/helper"

import { mockSingleBoardStore } from "../../../../common/util/board/input/test_helper"
import { restoreLoginId } from "../../login_id/kernel/convert"
import { restoreAuthUserField } from "../../account/kernel/convert"

import { initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import {
    LoadState,
    loadState_loaded,
    loadState_loading,
    mapLoadState,
} from "../../../../common/util/load/data"
import { OverwritePasswordAction, initOverwritePasswordAction } from "./action"

import { OverwritePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { SingleBoardStore } from "../../../../common/util/board/input/infra"

import { AuthUserAccount } from "../../account/kernel/data"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid new-password", async () => {
    const { overwrite, store } = standard()

    const result = observeAtom(overwrite.state)

    store.newPassword.set(VALID_PASSWORD.newPassword)

    await overwrite.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success" },
        { type: "initial" },
    ])
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { overwrite, store } = takeLongtime_elements()

    const result = observeAtom(overwrite.state)

    store.newPassword.set(VALID_PASSWORD.newPassword)

    await overwrite.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        { type: "success" },
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

    store.newPassword.set(VALID_PASSWORD.newPassword)

    overwrite.reset()

    expect(store.newPassword.get()).toEqual("")
})

function standard() {
    return initResource(standard_overwriteRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_overwriteRemote())
}

function initResource(overwritePasswordRemote: OverwritePasswordRemote): Readonly<{
    overwrite: OverwritePasswordAction
    store: Readonly<{
        newPassword: SingleBoardStore
    }>
}> {
    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })
    const data = mapAtom(list.state, (list) => mapLoadState(list, (list) => list[0]))
    const overwrite = initOverwritePasswordAction(data, {
        infra: {
            overwritePasswordRemote,
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
            newPassword: mockSingleBoardStore(overwrite.newPassword.input),
        },
    }
}

function standard_overwriteRemote(): OverwritePasswordRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_overwriteRemote(): OverwritePasswordRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
