import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverwriteLoginIdAction, initOverwriteLoginIdAction } from "./action"

import { restoreLoginId } from "../input/convert"

import { OverwriteLoginIdRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../kernel/data"

const VALID_LOGIN_ID = { newLoginId: "new-login-id" } as const

test("submit valid new-login-id", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.overwrite)

    await runner(async () => {
        store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

        return resource.overwrite.submit(user, (loginId) => {
            expect(loginId).toEqual("new-login-id")
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

    const runner = setupActionTestRunner(resource.overwrite)

    await runner(() => {
        store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

        return resource.overwrite.submit(user, (loginId) => {
            expect(loginId).toEqual("new-login-id")
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

test("submit without fields", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner(resource.overwrite)

    await runner(() => resource.overwrite.submit(user, () => null)).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("clear", () => {
    const { resource, store } = standard()

    store.newLoginId.set(VALID_LOGIN_ID.newLoginId)
    resource.overwrite.clear()

    expect(store.newLoginId.get()).toEqual("")
})

function standard() {
    return initResource(standard_overwriteRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_overwriteRemote())
}

function initResource(overwriteLoginIdRemote: OverwriteLoginIdRemote): Readonly<{
    resource: Readonly<{
        overwrite: OverwriteLoginIdAction
    }>
    store: Readonly<{
        newLoginId: BoardValueStore
    }>
    user: Readonly<{ loginId: LoginId }>
}> {
    const resource = {
        overwrite: initOverwriteLoginIdAction({
            infra: {
                overwriteLoginIdRemote,
            },
            config: {
                takeLongtimeThreshold: { wait_millisecond: 32 },
                resetToInitialTimeout: { wait_millisecond: 32 },
            },
        }),
    }

    const store = {
        newLoginId: mockBoardValueStore(resource.overwrite.newLoginId.input),
    }

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
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
