import { test, expect } from "vitest"
import { observeAtom } from "../../../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../../../common/util/timer/helper"
import { mockSingleBoardStore } from "../../../../../../common/util/board/input/test_helper"

import { restoreLoginId } from "../../../../login_id/kernel/convert"
import { restoreAuthUserField } from "../../../../account/kernel/convert"

import { initAtom, mapAtom } from "../../../../../../z_vendor/getto-atom/atom"
import {
    LoadState,
    loadState_loaded,
    loadState_loading,
    mapLoadState,
} from "../../../../../../common/util/load/data"
import { ChangeResetTokenDestinationAction, initChangeResetTokenDestinationAction } from "./action"
import { initLoadableListAtomUpdater } from "../../../../../../common/util/list/action"

import { ChangeResetTokenDestinationRemote } from "./infra"
import { SingleBoardStore } from "../../../../../../common/util/board/input/infra"

import { AuthUserAccount } from "../../../../account/kernel/data"

const VALID_INFO = {
    destinationType: "email",
    email: "user@example.com",
} as const

test("submit valid info", async () => {
    const { change, store } = standard()

    const result = observeAtom(change.state)

    store.destinationType.set(VALID_INFO.destinationType)
    store.email.set(VALID_INFO.email)

    await change.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        {
            type: "success",
            data: {
                loginId: "user-id",
                granted: [],
                resetTokenDestination: { type: "email", email: "user@example.com" },
                memo: restoreAuthUserField("initial-memo"),
            },
        },
        { type: "initial" },
    ])
})

test("submit valid login-id; take long time", async () => {
    // wait for take longtime timeout
    const { change, store } = takeLongtime_elements()

    const result = observeAtom(change.state)

    store.destinationType.set(VALID_INFO.destinationType)
    store.email.set(VALID_INFO.email)

    await change.submit()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        {
            type: "success",
            data: {
                loginId: "user-id",
                granted: [],
                resetTokenDestination: { type: "email", email: "user@example.com" },
                memo: restoreAuthUserField("initial-memo"),
            },
        },
        { type: "initial" },
    ])
})

test("submit with invalid value; empty email", async () => {
    const { change, store } = standard()

    const result = observeAtom(change.state)

    store.destinationType.set("email")
    store.email.set("")

    await change.submit()

    expect(result()).toEqual([])
})

test("reset", () => {
    const { change, store } = standard()

    store.destinationType.set(VALID_INFO.destinationType)
    store.email.set(VALID_INFO.email)

    change.reset()

    expect(store.destinationType.get()).toEqual("none")
    expect(store.email.get()).toEqual("")
})

function standard() {
    return initResource(standard_changeDestinationRemote())
}
function takeLongtime_elements() {
    return initResource(takeLongtime_changeDestinationRemote())
}

function initResource(modifyUserRemote: ChangeResetTokenDestinationRemote): Readonly<{
    change: ChangeResetTokenDestinationAction
    store: Readonly<{
        destinationType: SingleBoardStore
        email: SingleBoardStore
    }>
}> {
    const list = initAtom<LoadState<readonly AuthUserAccount[]>>({
        initialState: loadState_loading(),
    })
    const data = mapAtom(list.state, (list) => mapLoadState(list, (list) => list[0]))
    const change = initChangeResetTokenDestinationAction(data, initLoadableListAtomUpdater(list), {
        infra: {
            changeDestinationRemote: modifyUserRemote,
        },
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
            resetToInitialTimeout: { wait_millisecond: 32 },
        },
    })

    list.post(
        loadState_loaded([
            {
                loginId: restoreLoginId("user-id"),
                granted: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("initial-memo"),
            },
        ]),
    )

    return {
        change,
        store: {
            destinationType: mockSingleBoardStore(change.destination.type.input),
            email: mockSingleBoardStore(change.destination.email.input),
        },
    }
}

function standard_changeDestinationRemote(): ChangeResetTokenDestinationRemote {
    return async () => ({ success: true, value: true })
}
function takeLongtime_changeDestinationRemote(): ChangeResetTokenDestinationRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => ({ success: true, value: true }))
}
