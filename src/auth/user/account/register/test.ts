import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"
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

const VALID_INFO = {
    loginId: "login-id",
    grantedRoles: ["auth-user"],
    resetTokenDestinationEmail: "user@example.com",
    memo: "memo",
} as const

test("submit valid info", async () => {
    const { register, store } = standard()

    expect(
        await observeApplicationState(register.state, async () => {
            store.loginId.set(VALID_INFO.loginId)
            store.grantedRoles.set(VALID_INFO.grantedRoles)
            store.resetTokenDestinationType.set("email")
            store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)
            store.memo.set(VALID_INFO.memo)

            return register.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: "login-id",
                grantedRoles: ["auth-user"],
                resetTokenDestination: { type: "email", email: "user@example.com" },
                memo: "memo",
            },
        },
        { type: "initial" },
    ])
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { register, store } = takeLongtime_elements()

    expect(
        await observeApplicationState(register.state, async () => {
            store.loginId.set(VALID_INFO.loginId)
            store.grantedRoles.set(VALID_INFO.grantedRoles)
            store.resetTokenDestinationType.set("email")
            store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)
            store.memo.set(VALID_INFO.memo)

            return register.submit()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: "login-id",
                grantedRoles: ["auth-user"],
                resetTokenDestination: { type: "email", email: "user@example.com" },
                memo: "memo",
            },
        },
        { type: "initial" },
    ])
})

test("clear", () => {
    const { register, store } = standard()

    store.loginId.set(VALID_INFO.loginId)
    store.grantedRoles.set(VALID_INFO.grantedRoles)
    store.resetTokenDestinationType.set("email")
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)
    store.memo.set(VALID_INFO.memo)

    register.clear()

    expect(store.loginId.get()).toEqual("")
    expect(store.grantedRoles.get()).toEqual([])
    expect(store.resetTokenDestinationType.get()).toEqual("none")
    expect(store.resetTokenDestinationEmail.get()).toEqual("")
    expect(store.memo.get()).toEqual("")
})

function standard() {
    return initResource(standard_registerUserRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_registerUserRemote())
}

function initResource(registerUserRemote: RegisterAuthUserAccountRemote): Readonly<{
    register: RegisterAuthUserAccountAction
    store: Readonly<{
        loginId: BoardValueStore
        grantedRoles: MultipleBoardValueStore
        resetTokenDestinationType: BoardValueStore
        resetTokenDestinationEmail: BoardValueStore
        memo: BoardValueStore
    }>
}> {
    const register = initRegisterAuthUserAccountAction({
        infra: {
            registerUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    const store = {
        loginId: mockBoardValueStore(register.loginId.input),
        grantedRoles: mockMultipleBoardValueStore(register.grantedRoles.input),
        resetTokenDestinationType: mockBoardValueStore(
            register.resetTokenDestination.destinationType,
        ),
        resetTokenDestinationEmail: mockBoardValueStore(register.resetTokenDestination.email),
        memo: mockBoardValueStore(register.memo.input),
    }

    return {
        register,
        store,
    }
}

function standard_registerUserRemote(): RegisterAuthUserAccountRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_registerUserRemote(): RegisterAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
