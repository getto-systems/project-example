import { readSearchSort } from "./convert"

describe("sort", () => {
    test("read search sort", () => {
        const url = new URL("https://example.com/?search-sort-key=key&search-sort-order=normal")
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
        expect(
            readSearchSort(url.searchParams, "key", () => ({ found: false })),
        ).toEqual({
            key: "key",
            order: "normal",
        })
    })
})
