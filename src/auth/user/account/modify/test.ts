import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"
import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import {
    mockBoardValueStore,
    mockMultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/test_helper"

import { ModifyAuthUserAccountAction, initModifyAuthUserAccountAction } from "./action"

import { ModifyAuthUserAccountRemote, ModifyAuthUserAccountRemoteResult } from "./infra"
import {
    BoardValueStore,
    MultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/infra"

import { AuthUserAccountBasket } from "../kernel/data"
import { ModifyAuthUserAccountFields } from "./data"

const VALID_INFO = {
    grantedRoles: ["user"],
    destinationType: "email",
    email: "user@example.com",
} as const

test("submit valid info", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(async () => {
        store.grantedRoles.set(VALID_INFO.grantedRoles.map(markBoardValue))
        store.destinationType.set(markBoardValue(VALID_INFO.destinationType))
        store.email.set(markBoardValue(VALID_INFO.email))

        resource.modify.grantedRoles.grantedRoles.publisher.post()
        resource.modify.resetTokenDestination.destinationType.publisher.post()
        resource.modify.resetTokenDestination.input.publisher.post()

        return resource.modify.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try" },
            {
                type: "success",
                data: {
                    loginId: user.loginId,
                    grantedRoles: ["user"],
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store, user } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(() => {
        store.grantedRoles.set(VALID_INFO.grantedRoles.map(markBoardValue))
        store.destinationType.set(markBoardValue(VALID_INFO.destinationType))
        store.email.set(markBoardValue(VALID_INFO.email))

        resource.modify.grantedRoles.grantedRoles.publisher.post()
        resource.modify.resetTokenDestination.destinationType.publisher.post()
        resource.modify.resetTokenDestination.input.publisher.post()

        return resource.modify.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try" },
            { type: "take-longtime" },
            {
                type: "success",
                data: {
                    loginId: user.loginId,
                    grantedRoles: ["user"],
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
        ])
    })
})

test("submit with invalid value; empty email", async () => {
    const { resource, store, user } = standard()

    const runner = setupActionTestRunner(resource.modify.subscriber)

    await runner(() => {
        store.destinationType.set(markBoardValue("email"))
        store.email.set(markBoardValue(""))

        resource.modify.resetTokenDestination.destinationType.publisher.post()
        resource.modify.resetTokenDestination.input.publisher.post()

        return resource.modify.submit(user)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "failed", err: { type: "validation-error" } }])
    })
})

test("reset", () => {
    const { resource, store, user } = standard()

    store.grantedRoles.set(VALID_INFO.grantedRoles.map(markBoardValue))
    store.destinationType.set(markBoardValue(VALID_INFO.destinationType))
    store.email.set(markBoardValue(VALID_INFO.email))

    resource.modify.reset(user)

    expect(store.grantedRoles.get()).toEqual([])
    expect(store.destinationType.get()).toEqual("none")
    expect(store.email.get()).toEqual("")
})

test("terminate", async () => {
    const { resource, user } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.modify.subscriber.subscribe(handler)
            resource.modify.validate.subscriber.subscribe(handler)
            resource.modify.resetTokenDestination.validate.subscriber.subscribe(handler)
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
        destinationType: BoardValueStore
        email: BoardValueStore
    }>
    user: AuthUserAccountBasket
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
        grantedRoles: mockMultipleBoardValueStore(),
        destinationType: mockBoardValueStore(),
        email: mockBoardValueStore(),
    }

    resource.modify.grantedRoles.grantedRoles.connector.connect(store.grantedRoles)
    resource.modify.resetTokenDestination.destinationType.connector.connect(store.destinationType)
    resource.modify.resetTokenDestination.input.connector.connect(store.email)

    const user: AuthUserAccountBasket = {
        loginId: "user-id",
        grantedRoles: [],
        resetTokenDestination: { type: "none" },
    }

    return { resource, store, user }
}

function standard_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async (user, fields) => modifyUserRemote(user, fields)
}
function takeLongtime_modifyUserRemote(): ModifyAuthUserAccountRemote {
    return async (user, fields) =>
        ticker({ wait_millisecond: 64 }, () => modifyUserRemote(user, fields))
}
function modifyUserRemote(
    user: AuthUserAccountBasket,
    fields: ModifyAuthUserAccountFields,
): ModifyAuthUserAccountRemoteResult {
    return {
        success: true,
        value: {
            loginId: user.loginId,
            grantedRoles: fields.grantedRoles,
            resetTokenDestination: fields.resetTokenDestination,
        },
    }
}
