import { test, expect } from "vitest"

import { observeAtom } from "../../../../../z_vendor/getto-atom/test_helper"
import { mockMultipleBoardStore } from "../../../../../common/util/board/input/test_helper"

import { ALL_AUTH_PERMISSIONS } from "../../../../../x_content/permission"

import { initAtom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loaded } from "../../../../../common/util/load/data"
import { initAuthPermissionGrantedField } from "./action"

import { restoreAuthPermission } from "./convert"

import { AuthPermission } from "../../data"

test("restore auth-permission", () => {
    expect(restoreAuthPermission(ALL_AUTH_PERMISSIONS[0])).toEqual([ALL_AUTH_PERMISSIONS[0]])
})

test("invalid auth-permission", () => {
    expect(restoreAuthPermission("INVALID auth-permission")).toEqual([])
})

test("validate; valid input", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set(["auth-user"])

    expect(result()).toEqual([{ valid: true, value: ["auth-user"] }])
})

test("observe; has changed", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.observe)

    store.set(["auth-user"])

    expect(result()).toEqual([{ hasChanged: true }])
})

test("reset", () => {
    const { initializer, store } = standard()

    store.set(["auth-user"])
    initializer.reset()

    expect(store.get()).toEqual([])
})

function standard() {
    const options = initAtom<LoadState<readonly AuthPermission[]>>({
        initialState: loadState_loaded(ALL_AUTH_PERMISSIONS),
    })
    const [field, initializer] = initAuthPermissionGrantedField(options.state)
    const store = mockMultipleBoardStore(field.input)

    return { field, initializer, store }
}
