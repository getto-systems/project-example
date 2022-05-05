import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverwritePasswordAction, initOverwritePasswordAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { OverwritePasswordRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../../login_id/kernel/data"

const VALID_PASSWORD = { currentPassword: "current-password", newPassword: "new-password" } as const

test("submit valid new-password", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.overwrite.subscriber)

    await runner(async () => {
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return resource.overwrite.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try", hasTakenLongtime: false }, { type: "success" }])
    })
})

test("submit valid login-id and password; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.overwrite.subscriber)

    await runner(() => {
        store.newPassword.set(VALID_PASSWORD.newPassword)

        return resource.overwrite.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success" },
        ])
    })
})

test("submit without fields", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner(resource.overwrite.subscriber)

    await runner(() => resource.overwrite.submit(user)).then((stack) => {
        expect(stack).toEqual([])
    })
})

test("clear", () => {
    const { resource, store } = standard()

    store.newPassword.set(VALID_PASSWORD.newPassword)
    resource.overwrite.clear()

    expect(store.newPassword.get()).toEqual("")
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.overwrite.subscriber.subscribe(handler)
            resource.overwrite.validate.subscriber.subscribe(handler)
            resource.overwrite.newPassword.validate.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.overwrite.terminate()
        return resource.overwrite.submit(user)
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    return initResource(standard_overwriteRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_overwriteRemote())
}

function initResource(overwritePasswordRemote: OverwritePasswordRemote): Readonly<{
    resource: Readonly<{
        overwrite: OverwritePasswordAction
    }>
    store: Readonly<{
        newPassword: BoardValueStore
    }>
    user: Readonly<{ loginId: LoginId }>
}> {
    const resource = {
        overwrite: initOverwritePasswordAction({
            infra: {
                overwritePasswordRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        newPassword: mockBoardValueStore(resource.overwrite.newPassword.input),
    }

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
        },
    }
}

function standard_overwriteRemote(): OverwritePasswordRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_overwriteRemote(): OverwritePasswordRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
