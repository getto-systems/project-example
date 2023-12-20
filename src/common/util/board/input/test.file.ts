import { test, expect } from "vitest"

import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"

import { initFileBoard } from "./action"

test("file", () => {
    const board = initFileBoard()

    expect(board.value.currentState()).toEqual({ found: false })

    const result = observeAtom(board.value)

    board.input.onInput()

    expect(result()).toEqual([{ found: false }])
})
