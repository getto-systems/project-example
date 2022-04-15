import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverridePasswordAction, initOverridePasswordAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { OverridePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../../login_id/kernel/data"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid new-password", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.override.subscriber)

    await runner(async () => {
        store.newPassword.set(markBoardValue(VALID_PASSWORD.newPassword))

        return resource.override.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try" }, { type: "success" }])
    })
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.override.subscriber)

    await runner(() => {
        store.newPassword.set(markBoardValue(VALID_PASSWORD.newPassword))

        return resource.override.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try" }, { type: "take-longtime" }, { type: "success" }])
    })
})

test("submit without fields", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner(resource.override.subscriber)

    await runner(() => resource.override.submit(user)).then((stack) => {
        expect(stack).toEqual([{ type: "failed", err: { type: "validation-error" } }])
    })
})

test("clear", () => {
    const { resource, store } = standard()

    store.newPassword.set(markBoardValue(VALID_PASSWORD.newPassword))
    resource.override.clear()

    expect(store.newPassword.get()).toEqual("")
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.override.subscriber.subscribe(handler)
            resource.override.validate.subscriber.subscribe(handler)
            resource.override.newPassword.validate.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.override.terminate()
        return resource.override.submit(user)
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    return initResource(standard_overrideRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_overrideRemote())
}

function initResource(overridePasswordRemote: OverridePasswordRemote): Readonly<{
    resource: Readonly<{
        override: OverridePasswordAction
    }>
    store: Readonly<{
        newPassword: BoardValueStore
    }>
    user: Readonly<{ loginId: LoginId }>
}> {
    const resource = {
        override: initOverridePasswordAction({
            infra: {
                overridePasswordRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        currentPassword: mockBoardValueStore(),
        newPassword: mockBoardValueStore(),
    }

    resource.override.newPassword.input.connector.connect(store.newPassword)

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
        },
    }
}

function standard_overrideRemote(): OverridePasswordRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_overrideRemote(): OverridePasswordRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
