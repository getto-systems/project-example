import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../common/util/timer/helper"

import { mockSingleBoardStore } from "../../../../common/util/board/input/test_helper"

import { ChangePasswordAction, initChangePasswordAction } from "./action"

import { ChangePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { SingleBoardStore } from "../../../../common/util/board/input/infra"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid current-password and new-password", async () => {
    const { change, store } = standard()

    const result = observeAtom(change.state)

    store.currentPassword.set(VALID_PASSWORD.currentPassword)
    store.newPassword.set(VALID_PASSWORD.newPassword)

    await change.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success" },
        { type: "initial" },
    ])
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { change, store } = takeLongtime_elements()

    const result = observeAtom(change.state)

    store.currentPassword.set(VALID_PASSWORD.currentPassword)
    store.newPassword.set(VALID_PASSWORD.newPassword)

    await change.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        { type: "success" },
        { type: "initial" },
    ])
})

test("submit without fields", async () => {
    const { change } = standard()

    const result = observeAtom(change.state)

    await change.submit()

    expect(result()).toEqual([])
})

test("edit", () => {
    const { change, store } = standard()

    store.currentPassword.set(VALID_PASSWORD.currentPassword)
    store.newPassword.set(VALID_PASSWORD.newPassword)

    change.editable.open()

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
    change: ChangePasswordAction
    store: Readonly<{
        currentPassword: SingleBoardStore
        newPassword: SingleBoardStore
    }>
}> {
    const change = initChangePasswordAction({
        infra: {
            changePasswordRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    return {
        change,
        store: {
            currentPassword: mockSingleBoardStore(change.currentPassword.input),
            newPassword: mockSingleBoardStore(change.newPassword.input),
        },
    }
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
