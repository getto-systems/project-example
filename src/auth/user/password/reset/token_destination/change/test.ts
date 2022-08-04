import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../../z_lib/ui/timer/helper"
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

    const runner = setupActionTestRunner(change)

    await runner(async () => {
        store.destinationType.set(VALID_INFO.destinationType)
        store.email.set(VALID_INFO.email)

        return change.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            {
                type: "success",
                entry: {
                    loginId: "user-id",
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
            { type: "initial" },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { change, store } = takeLongtime_elements()

    const runner = setupActionTestRunner(change)

    await runner(() => {
        store.destinationType.set(VALID_INFO.destinationType)
        store.email.set(VALID_INFO.email)

        return change.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            {
                type: "success",
                entry: {
                    loginId: "user-id",
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
            { type: "initial" },
        ])
    })
})

test("submit with invalid value; empty email", async () => {
    const { change, store } = standard()

    const runner = setupActionTestRunner(change)

    await runner(() => {
        store.destinationType.set("email")
        store.email.set("")

        return change.submit()
    }).then((stack) => {
        expect(stack).toEqual([])
    })
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
