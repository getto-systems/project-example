import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverwritePasswordAction, initOverwritePasswordAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { OverwritePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid new-password", async () => {
    const { overwrite, store } = standard()

    const runner = setupActionTestRunner(overwrite)

    await runner(async () => {
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return overwrite.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success", entry: { loginId: "user-id" } },
            { type: "initial" },
        ])
    })
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { overwrite, store } = takeLongtime_elements()

    const runner = setupActionTestRunner(overwrite)

    await runner(() => {
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return overwrite.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success", entry: { loginId: "user-id" } },
            { type: "initial" },
        ])
    })
})

test("submit without fields", async () => {
    const { overwrite } = standard()

    const runner = setupActionTestRunner(overwrite)

    await runner(() => overwrite.submit()).then((stack) => {
        expect(stack).toEqual([])
    })
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
