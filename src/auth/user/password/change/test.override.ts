import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../common/util/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverwritePasswordAction, initOverwritePasswordAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { OverwritePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid new-password", async () => {
    const { overwrite, store } = standard()

    expect(
        await observeApplicationState(overwrite.state, async () => {
            store.newPassword.set(VALID_PASSWORD.newPassword)

            return overwrite.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", data: { loginId: "user-id" } },
        { type: "initial" },
    ])
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { overwrite, store } = takeLongtime_elements()

    expect(
        await observeApplicationState(overwrite.state, async () => {
            store.newPassword.set(VALID_PASSWORD.newPassword)

            return overwrite.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        { type: "success", data: { loginId: "user-id" } },
        { type: "initial" },
    ])
})

test("submit without fields", async () => {
    const { overwrite } = standard()

    expect(
        await observeApplicationState(overwrite.state, async () => {
            return overwrite.submit()
        }),
    ).toEqual([])
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
        newPassword: BoardValueStore
    }>
}> {
    const overwrite = initOverwritePasswordAction({
        infra: {
            overwritePasswordRemote,
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
            newPassword: mockBoardValueStore(overwrite.action.newPassword.input),
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
