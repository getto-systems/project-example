import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/init/mock"

import { ChangePasswordAction, initChangePasswordAction } from "./action"

import { ChangePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

describe("ChangePassword", () => {
    test("submit valid current-password and new-password", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.change.subscriber)

        await runner(async () => {
            store.currentPassword.set(markBoardValue(VALID_PASSWORD.currentPassword))
            store.newPassword.set(markBoardValue(VALID_PASSWORD.newPassword))

            return resource.change.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-change-password" },
                { type: "succeed-to-change-password" },
            ])
        })
    })

    test("submit valid login-id and password; take long time", async () => {
        // wait for take longtime timeout
        const { resource, store } = takeLongtime_elements()

        const runner = setupActionTestRunner(resource.change.subscriber)

        await runner(() => {
            store.currentPassword.set(markBoardValue(VALID_PASSWORD.currentPassword))
            store.newPassword.set(markBoardValue(VALID_PASSWORD.newPassword))

            return resource.change.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-change-password" },
                { type: "take-longtime-to-change-password" },
                { type: "succeed-to-change-password" },
            ])
        })
    })

    test("submit without fields", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.change.subscriber)

        await runner(() => resource.change.submit()).then((stack) => {
            expect(stack).toEqual([
                { type: "failed-to-change-password", err: { type: "validation-error" } },
            ])
        })
    })

    test("open; close", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.change.subscriber)

        await runner(async () => {
            resource.change.open()
            resource.change.close()
            return resource.change.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "input-password" }, { type: "initial-change-password" }])
        })
    })

    test("clear", () => {
        const { resource, store } = standard()

        store.currentPassword.set(markBoardValue(VALID_PASSWORD.currentPassword))
        store.newPassword.set(markBoardValue(VALID_PASSWORD.newPassword))
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
})

function standard() {
    return initResource(standard_change())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_change())
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
        currentPassword: mockBoardValueStore(),
        newPassword: mockBoardValueStore(),
    }

    resource.change.currentPassword.input.connector.connect(store.currentPassword)
    resource.change.newPassword.input.connector.connect(store.newPassword)

    return { resource, store }
}

function standard_change(): ChangePasswordRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_change(): ChangePasswordRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
