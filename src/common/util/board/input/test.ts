import { test, expect } from "vitest"

import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"

import { initFileBoard, initMultipleBoard, initSingleBoard } from "./action"

test("single", () => {
    const [board, initializer] = initSingleBoard()

    expect(board.value.currentState()).toEqual("")

    const result = observeAtom(board.value)

    initializer.init("initial")
    board.input.onInput()

    expect(result()).toEqual(["initial", "initial"])
})

test("multiple", () => {
    const [board, initializer] = initMultipleBoard()

    expect(board.value.currentState()).toEqual([])

    const result = observeAtom(board.value)

    initializer.init(["initial"])
    board.input.onInput()

    expect(result()).toEqual([["initial"], ["initial"]])
})

test("file", () => {
    const board = initFileBoard()

    expect(board.value.currentState()).toEqual({ found: false })

    const result = observeAtom(board.value)

    board.input.onInput()

    expect(result()).toEqual([{ found: false }])
})
