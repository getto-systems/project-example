import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { UnregisterAuthUserAccountAction, initUnregisterAuthUserAccountAction } from "./action"

import { restoreLoginId } from "../../login_id/input/convert"

import { UnregisterAuthUserAccountRemote } from "./infra"

test("submit", async () => {
    const { unregister } = standard()

    const runner = setupActionTestRunner(unregister)

    await runner(async () => {
        return unregister.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "success", entry: { loginId: "user-id" } },
        ])
    })
})

test("submit; take long time", async () => {
    // wait for take longtime timeout
    const { unregister } = takeLongtime_elements()

    const runner = setupActionTestRunner(unregister)

    await runner(() => {
        return unregister.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false },
            { type: "try", hasTakenLongtime: true },
            { type: "success", entry: { loginId: "user-id" } },
        ])
    })
})

function standard() {
    return initResource(standard_unregisterUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_unregisterUserRemote())
}

function initResource(modifyUserRemote: UnregisterAuthUserAccountRemote): Readonly<{
    unregister: UnregisterAuthUserAccountAction
}> {
    const unregister = initUnregisterAuthUserAccountAction({
        infra: {
            unregisterUserRemote: modifyUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
        },
    })

    unregister.handler.focus({
        loginId: restoreLoginId("user-id"),
    })

    return {
        unregister: unregister.action,
    }
}

function standard_unregisterUserRemote(): UnregisterAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_unregisterUserRemote(): UnregisterAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
