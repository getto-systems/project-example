import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"
import { mockMultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ModifyAuthUserAccountAction, initModifyAuthUserAccountAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { ModifyAuthUserAccountRemote } from "./infra"
import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { LoginId } from "../../login_id/kernel/data"
import { AuthRole } from "../../kernel/data"

const VALID_INFO = {
    grantedRoles: ["user"],
} as const

test("submit valid info", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(async () => {
        store.grantedRoles.set(VALID_INFO.grantedRoles)

        return resource.modify.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try" },
            { type: "success", data: { grantedRoles: ["user"] } },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(() => {
        store.grantedRoles.set(VALID_INFO.grantedRoles)

        return resource.modify.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try" },
            { type: "take-longtime" },
            { type: "success", data: { grantedRoles: ["user"] } },
        ])
    })
})

test("reset", () => {
    const { resource, store, user } = standard()

    store.grantedRoles.set(VALID_INFO.grantedRoles)

    resource.modify.reset(user)

    expect(store.grantedRoles.get()).toEqual([])
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.modify.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.modify.terminate()
        return resource.modify.submit(user)
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    return initResource(standard_modifyUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_modifyUserRemote())
}

function initResource(modifyUserRemote: ModifyAuthUserAccountRemote): Readonly<{
    resource: Readonly<{
        modify: ModifyAuthUserAccountAction
    }>
    store: Readonly<{
        grantedRoles: MultipleBoardValueStore
    }>
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>
}> {
    const resource = {
        modify: initModifyAuthUserAccountAction({
            infra: {
                modifyUserRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        grantedRoles: mockMultipleBoardValueStore(resource.modify.grantedRoles.grantedRoles),
    }

    return {
        resource,
        store,
        user: {
            loginId: restoreLoginId("user-id"),
            grantedRoles: [],
        },
    }
}

function standard_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
