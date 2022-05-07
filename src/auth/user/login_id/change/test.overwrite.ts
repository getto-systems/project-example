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

    const runner = setupActionTestRunner(resource.overwrite.subscriber)

    await runner(async () => {
        store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

        return resource.overwrite.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success", loginId: "new-login-id" },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.overwrite.subscriber)

    await runner(() => {
        store.newLoginId.set(VALID_LOGIN_ID.newLoginId)

        return resource.overwrite.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success", loginId: "new-login-id" },
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

    store.newLoginId.set(VALID_LOGIN_ID.newLoginId)
    resource.overwrite.clear()

    expect(store.newLoginId.get()).toEqual("")
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.overwrite.subscriber.subscribe(handler)
            resource.overwrite.validate.subscriber.subscribe(handler)
            resource.overwrite.newLoginId.validate.subscriber.subscribe(handler)
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
                takeLongtimeThreshold: { delay_millisecond: 32 },
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