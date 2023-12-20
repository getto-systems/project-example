import { test, expect } from "vitest"

import { mockSingleBoardStore, mockMultipleBoardStore } from "../input/test_helper"

import { initAtom } from "../../../../z_vendor/getto-atom/atom"
import { loadState_loaded, loadState_loading, LoadState } from "../../load/data"

import {
    composeSearchFilterBoard,
    initMultipleFilterBoard,
    initOffsetFilterBoard,
    initSelectFilterBoard,
    initTextFilterBoard,
    mapSelectFilterBoardFilter,
} from "./action"
import { SelectFilterBoardFilter, SingleFilterBoardValue } from "./data"
import {
    readMultipleFilterBoardFilter,
    readSelectFilterBoardFilter,
    readSingleFilterBoardValue,
    updateMultipleFilterBoardFilter,
    updateSelectFilterBoardFilter,
    updateSingleFilterBoardValue,
} from "./convert"

test("offset", async () => {
    const [offset, offsetInitializer] = initOffsetFilterBoard("0")

    const store = mockSingleBoardStore(offset.input)

    expect(offset.value.currentState()).toEqual("0")
    expect(offset.observe.currentState()).toEqual({ hasChanged: false })

    store.set("10")

    expect(offset.value.currentState()).toEqual("10")
    expect(offset.observe.currentState()).toEqual({ hasChanged: true })

    offsetInitializer.reset()

    expect(offset.value.currentState()).toEqual("0")
    expect(offset.observe.currentState()).toEqual({ hasChanged: false })

    offsetInitializer.init("100")

    expect(offset.value.currentState()).toEqual("100")
    expect(offset.observe.currentState()).toEqual({ hasChanged: false })

    store.set("1000")

    expect(offset.value.currentState()).toEqual("1000")
    expect(offset.observe.currentState()).toEqual({ hasChanged: true })

    offsetInitializer.pin()

    expect(offset.value.currentState()).toEqual("1000")
    expect(offset.observe.currentState()).toEqual({ hasChanged: false })
})

test("text", async () => {
    const [text, textInitializer] = initTextFilterBoard([])

    const store = mockSingleBoardStore(text.input)

    expect(text.filter.currentState()).toEqual([])
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    store.set("input")

    expect(text.filter.currentState()).toEqual(["input"])
    expect(text.observe.currentState()).toEqual({ hasChanged: true })

    textInitializer.reset()

    expect(text.filter.currentState()).toEqual([])
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    textInitializer.init(["initial"])

    expect(text.filter.currentState()).toEqual(["initial"])
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    store.set("input")

    expect(text.filter.currentState()).toEqual(["input"])
    expect(text.observe.currentState()).toEqual({ hasChanged: true })

    textInitializer.reset()

    expect(text.filter.currentState()).toEqual(["initial"])
    expect(text.observe.currentState()).toEqual({ hasChanged: false })

    store.set("input")

    expect(text.filter.currentState()).toEqual(["input"])
    expect(text.observe.currentState()).toEqual({ hasChanged: true })

    textInitializer.pin()

    expect(text.filter.currentState()).toEqual(["input"])
    expect(text.observe.currentState()).toEqual({ hasChanged: false })
})

test("select", async () => {
    const options = initAtom<LoadState<readonly string[]>>({ initialState: loadState_loading() })
    const [select, selectInitializer] = initSelectFilterBoard({
        initial: [],
        options: options.state,
        toFilter: (option) => option,
        toValue: (option) => option,
    })

    const store = mockSingleBoardStore(select.input)

    expect(select.filter.currentState()).toEqual([])
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    store.set("1")

    expect(select.filter.currentState()).toEqual([])
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    options.post(loadState_loaded(["1", "2", "3"]))

    expect(select.filter.currentState()).toEqual(["1"])
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    selectInitializer.reset()

    expect(select.filter.currentState()).toEqual([])
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    selectInitializer.init(["1"])

    expect(select.filter.currentState()).toEqual(["1"])
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    store.set("invalid")

    expect(select.filter.currentState()).toEqual([])
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    selectInitializer.reset()

    expect(select.filter.currentState()).toEqual(["1"])
    expect(select.observe.currentState()).toEqual({ hasChanged: false })

    store.set("3")

    expect(select.filter.currentState()).toEqual(["3"])
    expect(select.observe.currentState()).toEqual({ hasChanged: true })

    selectInitializer.pin()

    expect(select.filter.currentState()).toEqual(["3"])
    expect(select.observe.currentState()).toEqual({ hasChanged: false })
})

test("multiple", async () => {
    const options = initAtom<LoadState<readonly string[]>>({ initialState: loadState_loading() })
    const [multiple, multipleInitializer] = initMultipleFilterBoard({
        initial: [],
        options: options.state,
        toFilter: (option) => option,
        toValue: (option) => option,
    })

    const store = mockMultipleBoardStore(multiple.input)

    expect(multiple.filter.currentState()).toEqual([])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["1"])

    expect(multiple.filter.currentState()).toEqual([])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    options.post(loadState_loaded(["1", "2", "3"]))

    expect(multiple.filter.currentState()).toEqual(["1"])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    multipleInitializer.reset()

    expect(multiple.filter.currentState()).toEqual([])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    multipleInitializer.init(["1"])

    expect(multiple.filter.currentState()).toEqual(["1"])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["invalid"])

    expect(multiple.filter.currentState()).toEqual([])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    multipleInitializer.reset()

    expect(multiple.filter.currentState()).toEqual(["1"])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })

    store.set(["1", "2"])

    expect(multiple.filter.currentState()).toEqual(["1", "2"])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: true })

    multipleInitializer.pin()

    expect(multiple.filter.currentState()).toEqual(["1", "2"])
    expect(multiple.observe.currentState()).toEqual({ hasChanged: false })
})

test("compose", async () => {
    type Filter = Readonly<{
        a: SingleFilterBoardValue
        b: SingleFilterBoardValue
        c: SingleFilterBoardValue
    }>

    const filter: Filter = {
        a: ["valid"],
        b: ["valid"],
        c: ["valid"],
    }

    const offset = initOffsetFilterBoard("0")

    const a = initTextFilterBoard(filter.a)
    const b = initTextFilterBoard(filter.b)
    const c = initTextFilterBoard(filter.c)

    const { observe, reset, pin } = composeSearchFilterBoard(offset[0], [a, b, c])

    expect(observe.currentState()).toEqual({ hasChanged: false })

    const store = mockSingleBoardStore(a[0].input)

    store.set("invalid")

    expect(observe.currentState()).toEqual({ hasChanged: true })

    reset()

    expect(store.get()).toEqual("valid")

    expect(observe.currentState()).toEqual({ hasChanged: false })

    store.set("changed")

    expect(observe.currentState()).toEqual({ hasChanged: true })

    pin()

    expect(observe.currentState()).toEqual({ hasChanged: false })
})

test("map select filter", async () => {
    const filter: SelectFilterBoardFilter<string> = ["filter"]

    expect(mapSelectFilterBoardFilter(filter, (value) => [value])).toEqual([["filter"]])
})

test("read single filter board value", async () => {
    const url = new URL("http://localhost/index.html?filter=value")
    expect(readSingleFilterBoardValue(url.searchParams, "filter")).toEqual(["value"])
})

test("read select filter board value", async () => {
    const url = new URL("http://localhost/index.html?filter=value")
    expect(readSelectFilterBoardFilter(url.searchParams, "filter", (value) => [value])).toEqual([
        ["value"],
    ])
})

test("read multiple filter board value", async () => {
    const url = new URL("http://localhost/index.html?filter=value")
    expect(readMultipleFilterBoardFilter(url.searchParams, "filter", (value) => [value])).toEqual([
        ["value"],
    ])
})

test("update single filter board value", async () => {
    const url = new URL("http://localhost/index.html?filter=old")
    expect(updateSingleFilterBoardValue(url, "filter", ["value"])).toEqual(
        new URL("http://localhost/index.html?filter=value"),
    )
})

test("update select filter board value", async () => {
    const url = new URL("http://localhost/index.html?filter=old")
    expect(updateSelectFilterBoardFilter(url, "filter", ["value"], (value) => `${value}`)).toEqual(
        new URL("http://localhost/index.html?filter=value"),
    )
})

test("update multiple filter board value", async () => {
    const url = new URL("http://localhost/index.html?filter=old")
    expect(
        updateMultipleFilterBoardFilter(url, "filter", ["value"], (value) => `${value}`),
    ).toEqual(new URL("http://localhost/index.html?filter=value"))
})
