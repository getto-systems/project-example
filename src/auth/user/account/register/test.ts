import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"
import {
    mockBoardValueStore,
    mockMultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/test_helper"

import { RegisterAuthUserAccountAction, initRegisterAuthUserAccountAction } from "./action"

import { RegisterAuthUserAccountRemote } from "./infra"
import {
    BoardValueStore,
    MultipleBoardValueStore,
} from "../../../../z_vendor/getto-application/board/input/infra"
import { AuthUserAccount } from "../kernel/data"
import { restoreLoginId } from "../../login_id/input/convert"
import { restoreResetTokenDestination } from "../../password/reset/token_destination/kernel/convert"

const VALID_INFO = {
    loginId: "login-id",
    grantedRoles: ["user"],
    resetTokenDestinationEmail: "user@example.com",
} as const

test("submit valid info", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.register.subscriber)

    await runner(async () => {
        store.loginId.set(VALID_INFO.loginId)
        store.grantedRoles.set(VALID_INFO.grantedRoles)
        store.resetTokenDestinationType.set("email")
        store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)

        return resource.register.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try" },
            {
                type: "success",
                data: {
                    loginId: "login-id",
                    grantedRoles: ["user"],
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
        ])
    })
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { resource, store } = takeLongtime_elements()

    const runner = setupActionTestRunner(resource.register.subscriber)

    await runner(() => {
        store.loginId.set(VALID_INFO.loginId)
        store.grantedRoles.set(VALID_INFO.grantedRoles)
        store.resetTokenDestinationType.set("email")
        store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)

        return resource.register.submit()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try" },
            { type: "take-longtime" },
            {
                type: "success",
                data: {
                    loginId: "login-id",
                    grantedRoles: ["user"],
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
        ])
    })
})

test("clear", () => {
    const { resource, store } = standard()

    store.loginId.set(VALID_INFO.loginId)
    store.grantedRoles.set(VALID_INFO.grantedRoles)
    store.resetTokenDestinationType.set("email")
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)

    resource.register.clear()

    expect(store.loginId.get()).toEqual("")
    expect(store.grantedRoles.get()).toEqual([])
    expect(store.resetTokenDestinationType.get()).toEqual("none")
    expect(store.resetTokenDestinationEmail.get()).toEqual("")
})

test("focus / close", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.register.list.focused.subscriber)

    store.loginId.set(VALID_INFO.loginId)
    store.grantedRoles.set(VALID_INFO.grantedRoles)
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)

    await resource.register.submit()

    await runner(async () => {
        const user: AuthUserAccount = {
            loginId: restoreLoginId("login-id"),
            grantedRoles: [],
            resetTokenDestination: restoreResetTokenDestination({
                type: "email",
                email: "user@example.com",
            }),
        }
        const another: AuthUserAccount = {
            loginId: restoreLoginId("user-another"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
        }

        resource.register.list.focused.focus(user)
        expect(resource.register.list.focused.isFocused(user)).toBe(true)
        expect(resource.register.list.focused.isFocused(another)).toBe(false)

        resource.register.list.focused.close()
        expect(resource.register.list.focused.isFocused(user)).toBe(false)
        expect(resource.register.list.focused.isFocused(another)).toBe(false)

        return resource.register.list.focused.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "focus-on",
                user: {
                    loginId: "login-id",
                    grantedRoles: [],
                    resetTokenDestination: { type: "email", email: "user@example.com" },
                },
            },
            { type: "initial" },
        ])
    })
})

test("update user", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.register.list.focused.subscriber)

    store.loginId.set(VALID_INFO.loginId)
    store.grantedRoles.set(VALID_INFO.grantedRoles)
    store.resetTokenDestinationType.set("email")
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)

    await resource.register.submit()

    const user: AuthUserAccount = {
        loginId: restoreLoginId("login-id"),
        grantedRoles: [],
        resetTokenDestination: { type: "none" },
    }

    await runner(async () => {
        return resource.register.list.focused.update(user.loginId, user)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "focus-on", user }])
    })
})

test("terminate", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.register.subscriber.subscribe(handler)
            resource.register.validate.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.register.terminate()
        return resource.register.submit()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    return initResource(standard_registerUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_registerUserRemote())
}

function initResource(registerUserRemote: RegisterAuthUserAccountRemote): Readonly<{
    resource: Readonly<{
        register: RegisterAuthUserAccountAction
    }>
    store: Readonly<{
        loginId: BoardValueStore
        grantedRoles: MultipleBoardValueStore
        resetTokenDestinationType: BoardValueStore
        resetTokenDestinationEmail: BoardValueStore
    }>
}> {
    const resource = {
        register: initRegisterAuthUserAccountAction({
            infra: {
                registerUserRemote,
            },
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        loginId: mockBoardValueStore(resource.register.loginId.input),
        grantedRoles: mockMultipleBoardValueStore(resource.register.grantedRoles.grantedRoles),
        resetTokenDestinationType: mockBoardValueStore(
            resource.register.resetTokenDestination.destinationType,
        ),
        resetTokenDestinationEmail: mockBoardValueStore(
            resource.register.resetTokenDestination.email,
        ),
    }

    return {
        resource,
        store,
    }
}

function standard_registerUserRemote(): RegisterAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_registerUserRemote(): RegisterAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}