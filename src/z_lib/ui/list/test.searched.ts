import { test, expect } from "vitest"

import {
    observeApplicationState,
    observeApplicationStateTuple2,
    observeApplicationStateTuple3,
} from "../../../z_vendor/getto-application/action/test_helper"

import { ModifyFieldHandler } from "../modify/action"
import { initListSearchedAction, ListSearchedAction } from "./action"

import { DetectFocusListKeyResult, ListSearchedResult } from "./data"

test("initial search", async () => {
    const { list, data } = standard()

    expect(await list.state.ignitionState).toEqual({ isLoad: true, data })
})

test("error", async () => {
    const { list, data } = error()

    expect(await list.state.ignitionState).toEqual({ isLoad: true, data })
})

test("detected", async () => {
    const { list, item, stack } = detected()

    expect(
        await observeApplicationState(list.scroll.state, async () => {
            await list.state.ignitionState
            return list.scroll.state.currentState()
        }),
    ).toEqual([{ type: "detect" }])

    expect(stack).toEqual({
        focus: [item[0]],
        update: [],
        close: [],
    })
})

test("detect failed", async () => {
    const { list, stack } = detectFailed()

    expect(
        await observeApplicationState(list.focus.state, async () => {
            await list.state.ignitionState
            return list.focus.state.currentState()
        }),
    ).toEqual([{ type: "not-found" }])

    expect(stack).toEqual({
        focus: [],
        update: [],
        close: [],
    })
})

test("focus / close", async () => {
    const { list, item, stack } = standard()

    expect(
        await observeApplicationStateTuple2([list.focus.state, list.scroll.state], async () => {
            await list.state.ignitionState
            const another: Data = { id: 2, name: "another" }

            list.focus.change(item[0], { y: 0 })
            expect(list.focus.isFocused(item[0])).toBe(true)
            expect(list.focus.isFocused(another)).toBe(false)

            list.focus.close({ y: 1 })
            expect(list.focus.isFocused(item[0])).toBe(false)
            expect(list.focus.isFocused(another)).toBe(false)

            list.focus.change(another, { y: 2 })

            return list.focus.state.currentState()
        }),
    ).toEqual([
        [{ type: "focus-change", data: item[0] }, { type: "close" }, { type: "not-found" }],
        [
            { type: "focus-change", position: { y: 0 } },
            { type: "close", position: { y: 1 } },
        ],
    ])

    expect(stack).toEqual({
        focus: [item[0]],
        update: [],
        close: [true],
    })
})

test("update", async () => {
    const { list, item, stack } = standard()

    const updatedData: Data = { id: 1, name: "updated-name" }

    expect(
        await observeApplicationStateTuple2([list.state, list.focus.state], async () => {
            await list.state.ignitionState
            list.focus.change(item[0], { y: 0 })
            return list.focus.update(updatedData)
        }),
    ).toEqual([
        [
            {
                isLoad: true,
                data: { type: "success", response: { list: [updatedData, item[1]], sort: "id" } },
            },
        ],
        [
            { type: "focus-change", data: item[0] },
            { type: "data-update", data: updatedData },
        ],
    ])

    expect(stack).toEqual({
        focus: [item[0]],
        update: [updatedData],
        close: [],
    })
})

test("remove", async () => {
    const { list, item, stack } = standard()

    expect(
        await observeApplicationStateTuple3(
            [list.state, list.focus.state, list.scroll.state],
            async () => {
                await list.state.ignitionState
                list.focus.change(item[0], { y: 0 })
                return list.focus.remove()
            },
        ),
    ).toEqual([
        [
            {
                isLoad: true,
                data: { type: "success", response: { list: [item[1]], sort: "id" } },
            },
        ],
        [{ type: "focus-change", data: item[0] }, { type: "data-remove" }],
        [{ type: "focus-change", position: { y: 0 } }],
    ])

    expect(stack).toEqual({
        focus: [item[0]],
        update: [],
        close: [],
    })
})

function standard() {
    return initResource(detect_none, standard_result)
}
function detected() {
    return initResource(detect_exists, standard_result)
}
function detectFailed() {
    return initResource(detect_unknown, standard_result)
}
function error() {
    return initResource(detect_none, error_result)
}

function detect_none(): DetectFocusListKeyResult {
    return { found: false }
}
function detect_exists(): DetectFocusListKeyResult {
    return { found: true, key: "1" }
}
function detect_unknown(): DetectFocusListKeyResult {
    return { found: true, key: "unknown" }
}

function initResource<T>(
    detect: () => DetectFocusListKeyResult,
    result: () => [ListSearchedResult<Data, Summary, Error>, T],
): Readonly<{
    list: ListSearchedAction<Data, Summary, Error>
    data: ListSearchedResult<Data, Summary, Error>
    item: T
    stack: ModifyFieldStack
}> {
    const [data, item] = result()
    const list = initListSearchedAction<Data, Summary, Error>({
        initialSearch: Promise.resolve({ isLoad: true, data }),
        detect: {
            get: detect,
            key: (data) => `${data.id}`,
        },
    })
    const { stack, handler } = initModifyFieldStack()
    list.action.focus.onModify(handler)
    list.handler.load({ isLoad: true, data })

    return {
        list: list.action,
        data,
        item,
        stack,
    }
}

function standard_result(): [ListSearchedResult<Data, Summary, Error>, readonly Data[]] {
    const list = [
        { id: 1, name: "name" },
        { id: 2, name: "name-2" },
    ]
    return [
        {
            type: "success",
            response: { list, sort: "id" },
        },
        list,
    ]
}
function error_result(): [ListSearchedResult<Data, Summary, Error>, undefined] {
    return [
        {
            type: "failed",
            err: "error",
        },
        undefined,
    ]
}

type Data = Readonly<{ id: number; name: string }>
type Summary = Readonly<{ sort: string }>
type Error = string

type ModifyFieldStack = Readonly<{
    focus: readonly Data[]
    update: readonly Data[]
    close: readonly true[]
}>

function initModifyFieldStack(): Readonly<{
    handler: ModifyFieldHandler<Data>
    stack: ModifyFieldStack
}> {
    const stack = {
        focus: <Data[]>[],
        update: <Data[]>[],
        close: <true[]>[],
    }

    const handler: ModifyFieldHandler<Data> = {
        focus: (data) => stack.focus.push(data),
        update: (data) => stack.update.push(data),
        close: () => stack.close.push(true),
    }

    return { handler, stack }
}
