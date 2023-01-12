import { test, expect } from "vitest"
import { focusedData } from "./action"

test("focused data", () => {
    const data = { name: "data" }
    expect(focusedData({ type: "close", isFocused: false })).toEqual({ isFocused: false })
    expect(focusedData({ type: "close", isFocused: true, data })).toEqual({ isFocused: true, data })
    expect(focusedData({ type: "focus-change", data })).toEqual({ isFocused: true, data })
    expect(focusedData({ type: "data-update", data })).toEqual({ isFocused: true, data })
    expect(focusedData({ type: "data-remove" })).toEqual({ isFocused: false })
    expect(focusedData({ type: "not-found" })).toEqual({ isFocused: false })
})
