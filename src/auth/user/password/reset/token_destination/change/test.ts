import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../../common/util/timer/helper"
import { mockBoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/test_helper"

import { ChangeResetTokenDestinationAction, initChangeResetTokenDestinationAction } from "./action"

import { restoreLoginId } from "../../../../login_id/input/convert"

import { ChangeResetTokenDestinationRemote } from "./infra"
import { BoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/infra"

const VALID_INFO = {
    destinationType: "email",
    email: "user@example.com",
} as const

test("submit valid info", async () => {
    const { change, store } = standard()

    expect(
        await observeApplicationState(change.state, async () => {
            store.destinationType.set(VALID_INFO.destinationType)
            store.email.set(VALID_INFO.email)

            return change.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: "user-id",
                resetTokenDestination: { type: "email", email: "user@example.com" },
            },
        },
        { type: "initial" },
    ])
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { change, store } = takeLongtime_elements()

    expect(
        await observeApplicationState(change.state, async () => {
            store.destinationType.set(VALID_INFO.destinationType)
            store.email.set(VALID_INFO.email)

            return change.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: "user-id",
                resetTokenDestination: { type: "email", email: "user@example.com" },
            },
        },
        { type: "initial" },
    ])
})

test("submit with invalid value; empty email", async () => {
    const { change, store } = standard()

    expect(
        await observeApplicationState(change.state, async () => {
            store.destinationType.set("email")
            store.email.set("")

            return change.submit()
        }),
    ).toEqual([])
})

test("reset", () => {
    const { change, store } = standard()

    store.destinationType.set(VALID_INFO.destinationType)
    store.email.set(VALID_INFO.email)

    change.reset()

    expect(store.destinationType.get()).toEqual("none")
    expect(store.email.get()).toEqual("")
})

function standard() {
    return initResource(standard_changeDestinationRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_changeDestinationRemote())
}

function initResource(modifyUserRemote: ChangeResetTokenDestinationRemote): Readonly<{
    change: ChangeResetTokenDestinationAction
    store: Readonly<{
        destinationType: BoardValueStore
        email: BoardValueStore
    }>
}> {
    const change = initChangeResetTokenDestinationAction({
        infra: {
            changeDestinationRemote: modifyUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    change.handler.focus({
        loginId: restoreLoginId("user-id"),
        resetTokenDestination: { type: "none" },
    })

    return {
        change: change.action,
        store: {
            destinationType: mockBoardValueStore(change.action.destination.destinationType),
            email: mockBoardValueStore(change.action.destination.email),
        },
    }
}

function standard_changeDestinationRemote(): ChangeResetTokenDestinationRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_changeDestinationRemote(): ChangeResetTokenDestinationRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
