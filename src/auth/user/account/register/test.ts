import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../common/util/timer/helper"
import {
    mockSingleBoardStore,
    mockMultipleBoardStore,
} from "../../../../common/util/board/input/test_helper"

import { RegisterAuthUserAccountAction, initRegisterAuthUserAccountAction } from "./action"

import { RegisterAuthUserAccountRemote } from "./infra"
import { SingleBoardStore, MultipleBoardStore } from "../../../../common/util/board/input/infra"

const VALID_INFO = {
    loginId: "login-id",
    granted: ["auth-user"],
    resetTokenDestinationEmail: "user@example.com",
    memo: "memo",
} as const

test("submit valid info", async () => {
    const { register, store } = standard()

    const result = observeAtom(register.state)

    store.loginId.set(VALID_INFO.loginId)
    store.granted.set(VALID_INFO.granted)
    store.resetTokenDestinationType.set("email")
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)
    store.memo.set(VALID_INFO.memo)

    await register.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: "login-id",
                granted: ["auth-user"],
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

    const result = observeAtom(register.state)

    store.loginId.set(VALID_INFO.loginId)
    store.granted.set(VALID_INFO.granted)
    store.resetTokenDestinationType.set("email")
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)
    store.memo.set(VALID_INFO.memo)

    await register.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: "login-id",
                granted: ["auth-user"],
                resetTokenDestination: { type: "email", email: "user@example.com" },
                memo: "memo",
            },
        },
        { type: "initial" },
    ])
})

test("reset", () => {
    const { register, store } = standard()

    store.loginId.set(VALID_INFO.loginId)
    store.granted.set(VALID_INFO.granted)
    store.resetTokenDestinationType.set("email")
    store.resetTokenDestinationEmail.set(VALID_INFO.resetTokenDestinationEmail)
    store.memo.set(VALID_INFO.memo)

    register.reset()

    expect(store.loginId.get()).toEqual("")
    expect(store.granted.get()).toEqual([])
    expect(store.resetTokenDestinationType.get()).toEqual("")
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
        loginId: SingleBoardStore
        granted: MultipleBoardStore
        resetTokenDestinationType: SingleBoardStore
        resetTokenDestinationEmail: SingleBoardStore
        memo: SingleBoardStore
    }>
}> {
    const [register, _updater] = initRegisterAuthUserAccountAction({
        infra: {
            registerUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    const store = {
        loginId: mockSingleBoardStore(register.loginId.input),
        granted: mockMultipleBoardStore(register.granted.input),
        resetTokenDestinationType: mockSingleBoardStore(register.resetTokenDestination.type.input),
        resetTokenDestinationEmail: mockSingleBoardStore(
            register.resetTokenDestination.email.input,
        ),
        memo: mockSingleBoardStore(register.memo.input),
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
