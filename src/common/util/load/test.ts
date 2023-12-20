import { test, expect } from "vitest"

import { loadState_loaded, loadState_loading } from "./data"

test("loaded", async () => {
    expect(loadState_loaded("data")).toEqual({ isLoad: true, data: "data" })
})
test("loading", async () => {
    expect(loadState_loading()).toEqual({ isLoad: false })
})
