import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../../../z_lib/ui/timer/helper"
import { mockBoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/test_helper"

import { ChangeResetTokenDestinationAction, initChangeResetTokenDestinationAction } from "./action"

import { restoreLoginId } from "../../../../login_id/input/convert"

import { ChangeResetTokenDestinationRemote } from "./infra"
import { BoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../../../../login_id/kernel/data"
import { ResetTokenDestination } from "../kernel/data"

const VALID_INFO = {
    destinationType: "email",
    email: "user@example.com",
} as const

test("submit valid info", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.change.subscriber)

    await runner(async () => {
        store.destinationType.set(VALID_INFO.destinationType)
        store.email.set(VALID_INFO.email)

        return resource.change.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success", data: { type: "email", email: "user@example.com" } },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.change.subscriber)

    await runner(() => {
        store.destinationType.set(VALID_INFO.destinationType)
        store.email.set(VALID_INFO.email)

        return resource.change.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success", data: { type: "email", email: "user@example.com" } },
        ])
    })
})

test("submit with invalid value; empty email", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.change.subscriber)

    await runner(() => {
        store.destinationType.set("email")
        store.email.set("")

        return resource.change.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("reset", () => {
    const { resource, store, user } = standard()

    store.destinationType.set(VALID_INFO.destinationType)
    store.email.set(VALID_INFO.email)

    resource.change.reset(user.resetTokenDestination)

    expect(store.destinationType.get()).toEqual("none")
    expect(store.email.get()).toEqual("")
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.change.subscriber.subscribe(handler)
            resource.change.validate.subscriber.subscribe(handler)
            resource.change.destination.validate.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.change.terminate()
        return resource.change.submit(user)
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    return initResource(standard_changeDestinationRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_changeDestinationRemote())
}

function initResource(modifyUserRemote: ChangeResetTokenDestinationRemote): Readonly<{
    resource: Readonly<{
        change: ChangeResetTokenDestinationAction
    }>
    store: Readonly<{
        destinationType: BoardValueStore
        email: BoardValueStore
    }>
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>
}> {
    const resource = {
        change: initChangeResetTokenDestinationAction({
            infra: {
                changeDestinationRemote: modifyUserRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        destinationType: mockBoardValueStore(resource.change.destination.destinationType),
        email: mockBoardValueStore(resource.change.destination.email),
    }

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
            resetTokenDestination: { type: "none" },
        },
    }
}

function standard_changeDestinationRemote(): ChangeResetTokenDestinationRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_changeDestinationRemote(): ChangeResetTokenDestinationRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
