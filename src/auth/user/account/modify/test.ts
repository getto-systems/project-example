import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"
import {
    mockBoardValueStore,
    mockMultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ModifyAuthUserAccountAction, initModifyAuthUserAccountAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"
import { restoreAuthUserMemo } from "../input/memo/convert"

import { ModifyAuthUserAccountRemote } from "./infra"
import {
    BoardValueStore,
    MultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../../login_id/kernel/data"
import { ModifyAuthUserAccountFields } from "./data"

const VALID_INFO = {
    memo: "memo",
    grantedRoles: ["auth-user"],
} as const

test("submit valid info", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(async () => {
        store.memo.set(VALID_INFO.memo)
        store.grantedRoles.set(VALID_INFO.grantedRoles)

        return resource.modify.submit(user, (data) => {
            expect(data).toEqual({ grantedRoles: ["auth-user"], memo: "memo" })
        })
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success" },
            { type: "initial" },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(() => {
        store.memo.set(VALID_INFO.memo)
        store.grantedRoles.set(VALID_INFO.grantedRoles)

        return resource.modify.submit(user, (data) => {
            expect(data).toEqual({ grantedRoles: ["auth-user"], memo: "memo" })
        })
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success" },
            { type: "initial" },
        ])
    })
})

test("reset", () => {
    const { resource, store, user } = standard()

    store.memo.set(VALID_INFO.memo)
    store.grantedRoles.set(VALID_INFO.grantedRoles)

    resource.modify.reset(user)

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
    resource: Readonly<{
        modify: ModifyAuthUserAccountAction
    }>
    store: Readonly<{
        memo: BoardValueStore
        grantedRoles: MultipleBoardValueStore
    }>
    user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields
}> {
    const resource = {
        modify: initModifyAuthUserAccountAction({
            infra: {
                modifyUserRemote,
            },
            config: {
                takeLongtimeThreshold: { wait_millisecond: 32 },
                resetToInitialTimeout: { wait_millisecond: 32 },
            },
        }),
    }

    const store = {
        memo: mockBoardValueStore(resource.modify.memo.input),
        grantedRoles: mockMultipleBoardValueStore(resource.modify.grantedRoles.input),
    }

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
            grantedRoles: [],
            memo: restoreAuthUserMemo("initial-memo"),
        },
    }
}

function standard_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
