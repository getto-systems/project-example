import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ChangePasswordAction, initChangePasswordAction } from "./action"

import { ChangePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid current-password and new-password", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.change.state)

    await runner(async () => {
        store.currentPassword.set(VALID_PASSWORD.currentPassword)
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return resource.change.submit(() => null)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success" },
            { type: "initial" },
        ])
    })
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.change.state)

    await runner(() => {
        store.currentPassword.set(VALID_PASSWORD.currentPassword)
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return resource.change.submit(() => null)
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
    const { resource } = standard()

    const runner = setupActionTestRunner(resource.change.state)

    await runner(() => resource.change.submit(() => null)).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("edit", () => {
    const { resource, store } = standard()

    store.currentPassword.set(VALID_PASSWORD.currentPassword)
    store.newPassword.set(VALID_PASSWORD.newPassword)

    resource.change.edit()

    expect(store.currentPassword.get()).toEqual("")
    expect(store.newPassword.get()).toEqual("")
})

function standard() {
    return initResource(standard_changeRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_changeRemote())
}

function initResource(changePasswordRemote: ChangePasswordRemote): Readonly<{
    resource: Readonly<{
        change: ChangePasswordAction
    }>
    store: Readonly<{
        currentPassword: BoardValueStore
        newPassword: BoardValueStore
    }>
}> {
    const resource = {
        change: initChangePasswordAction({
            infra: {
                changePasswordRemote,
            },
            config: {
                takeLongtimeThreshold: { wait_millisecond: 32 },
                resetToInitialTimeout: { wait_millisecond: 32 },
            },
        }),
    }

    const store = {
        currentPassword: mockBoardValueStore(resource.change.currentPassword.input),
        newPassword: mockBoardValueStore(resource.change.newPassword.input),
    }

    return { resource, store }
}

function standard_changeRemote(): ChangePasswordRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_changeRemote(): ChangePasswordRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
