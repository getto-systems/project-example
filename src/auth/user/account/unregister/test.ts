import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../common/util/timer/helper"

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
import { UnregisterAuthUserAccountAction, initUnregisterAuthUserAccountAction } from "./action"

import { UnregisterAuthUserAccountRemote } from "./infra"

import { AuthUserAccount } from "../kernel/data"

test("submit", async () => {
    const { unregister } = standard()

    const result = observeAtom(unregister.state)

    await unregister.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: restoreLoginId("user-id"),
                granted: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("initial-memo"),
            },
        },
    ])
})

test("submit; take long time", async () => {
    // wait for take longtime timeout
    const { unregister } = takeLongtime_elements()

    const result = observeAtom(unregister.state)

    await unregister.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: restoreLoginId("user-id"),
                granted: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("initial-memo"),
            },
        },
    ])
})

function standard() {
    return initResource(standard_unregisterUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_unregisterUserRemote())
}

function initResource(modifyUserRemote: UnregisterAuthUserAccountRemote): Readonly<{
    unregister: UnregisterAuthUserAccountAction
}> {
    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })
    const data = mapAtom(list.state, (list) => mapLoadState(list, (list) => list[0]))
    const unregister = initUnregisterAuthUserAccountAction(
        data,
        initLoadableListAtomUpdater(list),
        {
            infra: {
                unregisterUserRemote: modifyUserRemote,
            },
            config: {
                takeLongtimeThreshold: { wait_millisecond: 32 },
            },
        },
    )

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
        unregister,
    }
}

function standard_unregisterUserRemote(): UnregisterAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_unregisterUserRemote(): UnregisterAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
