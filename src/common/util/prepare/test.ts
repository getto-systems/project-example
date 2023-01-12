import { test, expect } from "vitest"
import { prepared, preparing } from "./data"

test("prepared", async () => {
    expect(prepared("data")).toEqual({ isLoad: true, data: "data" })
})
test("preparing", async () => {
    expect(preparing()).toEqual({ isLoad: false })
})
