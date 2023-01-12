import { test, expect } from "vitest"

import {
    observeApplicationState,
    observeApplicationStateTuple2,
} from "../../../z_vendor/getto-application/action/test_helper"

import { ModifyFieldHandler } from "../modify/action"
import { initListRegisteredAction, ListRegisteredAction, ListRegisteredHandler } from "./action"

test("register", async () => {
    const { list, handler, stack } = standard()

    const data: Data = { id: 1, name: "name" }

    expect(
        await observeApplicationState(list.state, async () => {
            return handler.register(data)
        }),
    ).toEqual([{ isLoad: true, data: [data] }])

    expect(stack).toEqual({
        focus: [],
        update: [],
        close: [],
    })
})

test("focus / close", async () => {
    const { list, handler, stack } = standard()

    const data: Data = { id: 1, name: "name" }
    handler.register(data)

    expect(
        await observeApplicationState(list.focus.state, async () => {
            const another: Data = { id: 2, name: "another" }

            list.focus.change(data)
            list.focus.close()
            list.focus.change(another)

            return list.focus.state.currentState()
        }),
    ).toEqual([
        { type: "focus-change", data },
        { type: "close", isFocused: true, data },
        { type: "not-found" },
    ])

    expect(stack).toEqual({
        focus: [data],
        update: [],
        close: [true],
    })
})

test("update", async () => {
    const { list, handler, stack } = standard()

    const data: Data = { id: 1, name: "name" }
    const updatedData: Data = { id: 1, name: "updated-name" }

    expect(
        await observeApplicationStateTuple2([list.state, list.focus.state], async () => {
            handler.register(data)
            list.focus.change(data)
            return list.focus.update(updatedData)
        }),
    ).toEqual([
        [
            { isLoad: true, data: [data] },
            { isLoad: true, data: [updatedData] },
        ],
        [
            { type: "focus-change", data },
            { type: "data-update", data: updatedData },
        ],
    ])

    expect(stack).toEqual({
        focus: [data],
        update: [updatedData],
        close: [],
    })
})

test("remove", async () => {
    const { list, handler, stack } = standard()

    const data: Data = { id: 1, name: "name" }

    expect(
        await observeApplicationStateTuple2([list.state, list.focus.state], async () => {
            handler.register(data)
            list.focus.change(data)
            return list.focus.remove()
        }),
    ).toEqual([
        [
            { isLoad: true, data: [data] },
            { isLoad: true, data: [] },
        ],
        [{ type: "focus-change", data }, { type: "data-remove" }],
    ])

    expect(stack).toEqual({
        focus: [data],
        update: [],
        close: [],
    })
})

function standard(): Readonly<{
    list: ListRegisteredAction<Data>
    handler: ListRegisteredHandler<Data>
    stack: ModifyFieldStack
}> {
    const list = initListRegisteredAction<Data>()
    const { stack, handler } = initModifyFieldStack()
    list.action.focus.onModify(handler)

    return {
        list: list.action,
        handler: list.handler,
        stack,
    }
}

type Data = Readonly<{ id: number; name: string }>

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