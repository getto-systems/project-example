import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ChangePasswordAction, initChangePasswordAction } from "./action"

import { ChangePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid current-password and new-password", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.change.subscriber)

    await runner(async () => {
        store.currentPassword.set(VALID_PASSWORD.currentPassword)
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return resource.change.submit()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try" }, { type: "success" }])
    })
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.change.subscriber)

    await runner(() => {
        store.currentPassword.set(VALID_PASSWORD.currentPassword)
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return resource.change.submit()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try" }, { type: "take-longtime" }, { type: "success" }])
    })
})

test("submit without fields", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner(resource.change.subscriber)

    await runner(() => resource.change.submit()).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("clear", () => {
    const { resource, store } = standard()

    store.currentPassword.set(VALID_PASSWORD.currentPassword)
    store.newPassword.set(VALID_PASSWORD.newPassword)
    resource.change.clear()

    expect(store.currentPassword.get()).toEqual("")
    expect(store.newPassword.get()).toEqual("")
})

test("terminate", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.change.subscriber.subscribe(handler)
            resource.change.validate.subscriber.subscribe(handler)
            resource.change.currentPassword.validate.subscriber.subscribe(handler)
            resource.change.newPassword.validate.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.change.terminate()
        return resource.change.submit()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
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
                takeLongtimeThreshold: { delay_millisecond: 32 },
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
