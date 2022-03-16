import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ModifyAuthUserAccountAction, initModifyAuthUserAccountAction } from "./action"

import { ModifyAuthUserAccountRemote, ModifyAuthUserAccountRemoteResult } from "./infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { AuthUserAccountBasket } from "../kernel/data"

const VALID_INFO = { email: "user@example.com" } as const

describe("ModifyAuthUserAccount", () => {
    test("submit valid info", async () => {
        const { resource, store, user } = standard()

        const runner = setupActionTestRunner(resource.override.subscriber)

        await runner(async () => {
            store.email.set(markBoardValue(VALID_INFO.email))

            return resource.override.submit(user)
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-override-login-id" },
                { type: "succeed-to-override-login-id" },
            ])
        })
    })

    test("submit valid login-id; take long time", async () => {
        // wait for take longtime timeout
        const { resource, store, user } = takeLongtime_elements()

        const runner = setupActionTestRunner(resource.override.subscriber)

        await runner(() => {
            store.email.set(markBoardValue(VALID_INFO.email))

            return resource.override.submit(user)
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-override-login-id" },
                { type: "take-longtime-to-override-login-id" },
                { type: "succeed-to-override-login-id" },
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

    test("reset", () => {
        const { resource, store, user } = standard()

        store.email.set(markBoardValue(VALID_INFO.email))
        resource.override.reset(user)

        expect(store.email.get()).toEqual("")
    })

    test("terminate", async () => {
        const { resource, user } = standard()

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                resource.override.subscriber.subscribe(handler)
                resource.override.validate.subscriber.subscribe(handler)
                resource.override.resetTokenDestination.validate.subscriber.subscribe(handler)
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

function initResource(overrideLoginIdRemote: ModifyAuthUserAccountRemote): Readonly<{
    resource: Readonly<{
        override: ModifyAuthUserAccountAction
    }>
    store: Readonly<{
        email: BoardValueStore
    }>
    user: AuthUserAccountBasket
}> {
    const resource = {
        override: initModifyAuthUserAccountAction({
            infra: {
                modifyUserRemote: overrideLoginIdRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        email: mockBoardValueStore(),
    }

    resource.override.resetTokenDestination.input.connector.connect(store.email)

    const user: AuthUserAccountBasket = {
        loginId: "user-id",
        grantedRoles: [],
        resetTokenDestination: { type: "none" },
    }

    return { resource, store, user }
}

function standard_overrideRemote(): ModifyAuthUserAccountRemote {
    return async () => standard_changeRemoteResult()
}
function takeLongtime_overrideRemote(): ModifyAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_changeRemoteResult())
}
function standard_changeRemoteResult(): ModifyAuthUserAccountRemoteResult {
    return {
        success: true,
        // TODO 適切な user を返す
        value: { loginId: "user-id", grantedRoles: [], resetTokenDestination: { type: "none" } },
    }
}
