import { test, expect } from "vitest"
import { readSearchSort } from "./convert"

test("read search sort", () => {
    const url = new URL("https://example.com/?search-sort-key=key&search-sort-order=normal")
    expect(
        readSearchSort(url.searchParams, "key", (value) => ({ found: true, key: value })),
    ).toEqual({
        key: "key",
        order: "normal",
    })
})
test("read search sort; reverse", () => {
    const url = new URL("https://example.com/?search-sort-key=key&search-sort-order=reverse")
    expect(
        readSearchSort(url.searchParams, "key", (value) => ({ found: true, key: value })),
    ).toEqual({
        key: "key",
        order: "reverse",
    })
})
test("read search sort; unknown order", () => {
    const url = new URL("https://example.com/?search-sort-key=key&search-sort-order=unknown")
    expect(
        readSearchSort(url.searchParams, "key", (value) => ({ found: true, key: value })),
    ).toEqual({
        key: "key",
        order: "normal",
    })
})
test("read search sort; not found", () => {
    const url = new URL("https://example.com/")
    expect(
        readSearchSort(url.searchParams, "key", (value) => ({ found: true, key: value })),
    ).toEqual({
        key: "key",
        order: "normal",
    })
})
test("read search sort; invalid", () => {
    const url = new URL("https://example.com/?search-sort-key=key&search-sort-order=normal")
    expect(readSearchSort(url.searchParams, "key", () => ({ found: false }))).toEqual({
        key: "key",
        order: "normal",
    })
})
