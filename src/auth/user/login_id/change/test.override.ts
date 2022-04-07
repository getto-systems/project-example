import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { OverrideLoginIdAction, initOverrideLoginIdAction } from "./action"

import { restoreLoginId } from "../input/convert"

import { OverrideLoginIdRemote, ChangePasswordRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../input/data"

const VALID_LOGIN_ID = { newLoginId: "new-login-id" } as const

describe("OverrideLoginId", () => {
    test("submit valid new-login-id", async () => {
        const { resource, store, user } = standard()

        const runner = setupActionTestRunner(resource.override.subscriber)

        await runner(async () => {
            store.newLoginId.set(markBoardValue(VALID_LOGIN_ID.newLoginId))

            return resource.override.submit(user)
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-override-login-id" },
                { type: "succeed-to-override-login-id", loginId: "new-login-id" },
            ])
        })
    })

    test("submit valid login-id; take long time", async () => {
        // wait for take longtime timeout
        const { resource, store, user } = takeLongtime_elements()

        const runner = setupActionTestRunner(resource.override.subscriber)

        await runner(() => {
            store.newLoginId.set(markBoardValue(VALID_LOGIN_ID.newLoginId))

            return resource.override.submit(user)
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-override-login-id" },
                { type: "take-longtime-to-override-login-id" },
                { type: "succeed-to-override-login-id", loginId: "new-login-id" },
            ])
        })
    })

    test("submit without fields", async () => {
        const { resource, user } = standard()

        const runner = setupActionTestRunner(resource.override.subscriber)

        await runner(() => resource.override.submit(user)).then((stack) => {
            expect(stack).toEqual([
                { type: "failed-to-override-login-id", err: { type: "validation-error" } },
            ])
        })
    })

    test("clear", () => {
        const { resource, store } = standard()

        store.newLoginId.set(markBoardValue(VALID_LOGIN_ID.newLoginId))
        resource.override.clear()

        expect(store.newLoginId.get()).toEqual("")
    })

    test("terminate", async () => {
        const { resource, user } = standard()

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                resource.override.subscriber.subscribe(handler)
                resource.override.validate.subscriber.subscribe(handler)
                resource.override.newLoginId.validate.subscriber.subscribe(handler)
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
})

function standard() {
    return initResource(standard_overrideRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_overrideRemote())
}

function initResource(overrideLoginIdRemote: OverrideLoginIdRemote): Readonly<{
    resource: Readonly<{
        override: OverrideLoginIdAction
    }>
    store: Readonly<{
        newLoginId: BoardValueStore
    }>
    user: Readonly<{ loginId: LoginId }>
}> {
    const resource = {
        override: initOverrideLoginIdAction({
            infra: {
                overrideLoginIdRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        newLoginId: mockBoardValueStore(),
    }

    resource.override.newLoginId.input.connector.connect(store.newLoginId)

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
        },
    }
}

function standard_overrideRemote(): OverrideLoginIdRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_overrideRemote(): OverrideLoginIdRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ChangePasswordRemoteResult {
    return {
        success: true,
        value: true,
    }
}
