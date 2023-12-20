import { test, expect } from "vitest"

import { mockSingleBoardStore, mockMultipleBoardStore } from "../input/test_helper"

import { initAtom } from "../../../../z_vendor/getto-atom/atom"
import { loadState_loaded, loadState_loading, LoadState } from "../../load/data"

import {
    composeModifyFieldBoard,
    composeRegisterFieldBoard,
    initMultipleFieldBoard,
    initSelectFieldBoard,
    initTextFieldBoard,
    initVectorFieldBoard,
} from "./action"

import { ValidateBoardValue } from "../validate/data"

test("text", async () => {
    const [text, textInitializer] = initTextFieldBoard({
        convert: (value) => {
            if (value === "valid") {
                return { valid: true, value }
            } else {
                return { valid: false, err: `err: ${value}` }
            }
        },
    })

    const store = mockSingleBoardStore(text.input)

    expect(text.validate.currentState()).toEqual({ valid: false, err: "err: " })
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    store.set("valid")

    expect(text.validate.currentState()).toEqual({ valid: true, value: "valid" })
    expect(text.observe.currentState()).toEqual({ hasChanged: true })

    textInitializer.reset()

    expect(text.validate.currentState()).toEqual({ valid: false, err: "err: " })
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    textInitializer.init("valid")

    expect(text.validate.currentState()).toEqual({ valid: true, value: "valid" })
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    store.set("invalid")

    expect(text.validate.currentState()).toEqual({ valid: false, err: "err: invalid" })
    expect(text.observe.currentState()).toEqual({ hasChanged: true })

    textInitializer.reset()

    expect(text.validate.currentState()).toEqual({ valid: true, value: "valid" })
    expect(text.observe.currentState()).toEqual({ hasChanged: false })
})

test("vector", async () => {
    const [vector, vectorInitializer] = initVectorFieldBoard({
        convert: (value) => {
            if (value === "valid") {
                return { valid: true, value }
            } else {
                return { valid: false, err: `err: ${value}` }
            }
        },
    })

    const store = mockMultipleBoardStore(vector.input)

    expect(vector.validate.currentState()).toEqual({ valid: true, value: [] })
    expect(vector.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["valid"])

    expect(vector.validate.currentState()).toEqual({ valid: true, value: ["valid"] })
    expect(vector.observe.currentState()).toEqual({ hasChanged: true })

    vectorInitializer.reset()

    expect(vector.validate.currentState()).toEqual({ valid: true, value: [] })
    expect(vector.observe.currentState()).toEqual({ hasChanged: false })

    vectorInitializer.init(["valid"])

    expect(vector.validate.currentState()).toEqual({ valid: true, value: ["valid"] })
    expect(vector.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["invalid"])

    expect(vector.validate.currentState()).toEqual({ valid: false, err: ["err: invalid"] })
    expect(vector.observe.currentState()).toEqual({ hasChanged: true })

    vectorInitializer.reset()

    expect(vector.validate.currentState()).toEqual({ valid: true, value: ["valid"] })
    expect(vector.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["invalid", "unknown"])

    expect(vector.validate.currentState()).toEqual({
        valid: false,
        err: ["err: invalid", "err: unknown"],
    })
    expect(vector.observe.currentState()).toEqual({ hasChanged: true })
})

test("select", async () => {
    const options = initAtom<LoadState<readonly string[]>>({ initialState: loadState_loading() })
    const [select, selectInitializer] = initSelectFieldBoard(options.state, {
        convert: (value) => value,
    })

    const store = mockSingleBoardStore(select.input)

    expect(select.validate.currentState()).toEqual({ valid: false, err: { type: "not-loaded" } })
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    store.set("1")

    expect(select.validate.currentState()).toEqual({ valid: false, err: { type: "not-loaded" } })
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    options.post(loadState_loaded(["1", "2", "3"]))

    expect(select.validate.currentState()).toEqual({ valid: true, value: "1" })
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    selectInitializer.reset()

    expect(select.validate.currentState()).toEqual({ valid: false, err: { type: "not-selected" } })
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    selectInitializer.init("1")

    expect(select.validate.currentState()).toEqual({ valid: true, value: "1" })
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    store.set("invalid")

    expect(select.validate.currentState()).toEqual({ valid: false, err: { type: "not-selected" } })
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    selectInitializer.reset()

    expect(select.validate.currentState()).toEqual({ valid: true, value: "1" })
    expect(select.observe.currentState()).toEqual({ hasChanged: false })
})

test("multiple", async () => {
    const options = initAtom<LoadState<readonly string[]>>({ initialState: loadState_loading() })
    const [multiple, multipleInitializer] = initMultipleFieldBoard(options.state, {
        convert: (value) => value,
    })

    const store = mockMultipleBoardStore(multiple.input)

    expect(multiple.validate.currentState()).toEqual({ valid: false, err: { type: "not-loaded" } })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["1"])

    expect(multiple.validate.currentState()).toEqual({ valid: false, err: { type: "not-loaded" } })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    options.post(loadState_loaded(["1", "2", "3"]))

    expect(multiple.validate.currentState()).toEqual({ valid: true, value: ["1"] })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    multipleInitializer.reset()

    expect(multiple.validate.currentState()).toEqual({ valid: true, value: [] })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    multipleInitializer.init(["1"])

    expect(multiple.validate.currentState()).toEqual({ valid: true, value: ["1"] })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["invalid"])

    expect(multiple.validate.currentState()).toEqual({
        valid: false,
        err: { type: "not-selected" },
    })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    multipleInitializer.reset()

    expect(multiple.validate.currentState()).toEqual({ valid: true, value: ["1"] })
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })
})

test("register", async () => {
    const convert = (value: string): ValidateBoardValue<string, string> => {
        if (value === "valid") {
            return { valid: true, value }
        } else {
            return { valid: false, err: `err: ${value}` }
        }
    }
    const a = initTextFieldBoard({ convert })
    const b = initTextFieldBoard({ convert })
    const c = initTextFieldBoard({ convert })
    const form = composeRegisterFieldBoard([a, b, c])

    expect(form.validate.currentState()).toEqual({ valid: false })
    expect(form.observe.currentState()).toEqual({ hasChanged: false })

    const store = mockSingleBoardStore(a[0].input)

    store.set("valid")

    expect(form.validate.currentState()).toEqual({ valid: false })
    expect(form.observe.currentState()).toEqual({ hasChanged: true })

    form.reset()

    expect(store.get()).toEqual("")

    expect(form.validate.currentState()).toEqual({ valid: false })
    expect(form.observe.currentState()).toEqual({ hasChanged: false })
})

test("modify", async () => {
    type Data = Readonly<{
        a: string
        b: string
        c: string
    }>

    const data = initAtom<LoadState<Data>>({ initialState: loadState_loading() })
    const convert = (value: string): ValidateBoardValue<string, string> => {
        if (value === "valid") {
            return { valid: true, value }
        } else {
            return { valid: false, err: `err: ${value}` }
        }
    }
    const a = initTextFieldBoard({ convert })
    const b = initTextFieldBoard({ convert })
    const c = initTextFieldBoard({ convert })
    const form = composeModifyFieldBoard(data.state, [
        [a, (data: Data) => data.a],
        [b, (data: Data) => data.b],
        [c, (data: Data) => data.c],
    ])

    expect(form.editable.state.currentState()).toEqual({ isEditable: false })
    expect(form.validate.currentState()).toEqual({ valid: false })
    expect(form.observe.currentState()).toEqual({ hasChanged: false })

    data.post(
        loadState_loaded({
            a: "valid",
            b: "valid",
            c: "valid",
        }),
    )

    expect(form.editable.state.currentState()).toEqual({ isEditable: false })
    expect(form.validate.currentState()).toEqual({ valid: true })
    expect(form.observe.currentState()).toEqual({ hasChanged: false })

    const store = mockSingleBoardStore(a[0].input)

    form.editable.open()

    store.set("invalid")

    expect(form.editable.state.currentState()).toEqual({ isEditable: true })
    expect(form.validate.currentState()).toEqual({ valid: false })
    expect(form.observe.currentState()).toEqual({ hasChanged: true })

    form.reset()

    expect(store.get()).toEqual("valid")

    expect(form.editable.state.currentState()).toEqual({ isEditable: true })
    expect(form.validate.currentState()).toEqual({ valid: true })
    expect(form.observe.currentState()).toEqual({ hasChanged: false })

    store.set("invalid")
    form.editable.close()
    form.editable.open()

    expect(store.get()).toEqual("valid")

    expect(form.editable.state.currentState()).toEqual({ isEditable: true })
    expect(form.validate.currentState()).toEqual({ valid: true })
    expect(form.observe.currentState()).toEqual({ hasChanged: false })
})
