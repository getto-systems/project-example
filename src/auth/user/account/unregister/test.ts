import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { UnregisterAuthUserAccountAction, initUnregisterAuthUserAccountAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { UnregisterAuthUserAccountRemote } from "./infra"

import { LoginId } from "../../login_id/kernel/data"

test("submit", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner(resource.unregister.subscriber)

    await runner(async () => {
        return resource.unregister.submit(user, () => null)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "try", hasTakenLongtime: false }, { type: "success" }])
    })
})

test("submit; take long time", async () => {
    // wait for take longtime timeout
    const { resource, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.unregister.subscriber)

    await runner(() => {
        return resource.unregister.submit(user, () => null)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success" },
        ])
    })
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.unregister.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.unregister.terminate()
        return resource.unregister.submit(user, () => null)
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    return initResource(standard_unregisterUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_unregisterUserRemote())
}

function initResource(modifyUserRemote: UnregisterAuthUserAccountRemote): Readonly<{
    resource: Readonly<{
        unregister: UnregisterAuthUserAccountAction
    }>
    user: Readonly<{ loginId: LoginId }>
}> {
    const resource = {
        unregister: initUnregisterAuthUserAccountAction({
            infra: {
                unregisterUserRemote: modifyUserRemote,
            },
            config: {
                takeLongtimeThreshold: { wait_millisecond: 32 },
            },
        }),
    }

    return {
        resource,
        user: {
            loginId: restoreLoginId("user-id"),
        },
    }
}

function standard_unregisterUserRemote(): UnregisterAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_unregisterUserRemote(): UnregisterAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
