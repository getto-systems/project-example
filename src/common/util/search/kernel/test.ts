import { test, expect } from "vitest"
import {
    parseSearchPage,
    readSearchMultipleValueFilter,
    readSearchSelectResult,
    readSearchTextFilter,
    updateSearchMultipleValueFilter,
    updateSearchSelectResult,
    updateSearchTextFilter,
} from "./convert"

test("parse search page", () => {
    expect(parseSearchPage({ count: 10, limit: 10, offset: 10 })).toEqual({
        count: 10,
        limit: 10,
        offset: 10,
    })
})
test("parse search page; default", () => {
    expect(parseSearchPage(null)).toEqual({
        count: 0,
        limit: 0,
        offset: 0,
    })
})

test("read single value filter", () => {
    const url = new URL("https://example.com/?key=search")
    expect(readSearchTextFilter(url.searchParams, "key")).toEqual({
        filter: true,
        value: "search",
    })
})
test("read single value filter; not found", () => {
    const url = new URL("https://example.com/")
    expect(readSearchTextFilter(url.searchParams, "key")).toEqual({
        filter: false,
    })
})

test("read multiple value filter", () => {
    const url = new URL("https://example.com/?key=search-1&key=search-2")
    expect(readSearchMultipleValueFilter(url.searchParams, "key")).toEqual(["search-1", "search-2"])
})
test("read multiple value filter; not found", () => {
    const url = new URL("https://example.com/")
    expect(readSearchMultipleValueFilter(url.searchParams, "key")).toEqual([])
})

test("update single value filter", () => {
    const url = new URL("https://example.com/")
    expect(
        updateSearchTextFilter(url, "key", {
            filter: true,
            value: "search",
        }).toString(),
    ).toEqual("https://example.com/?key=search")
})
test("update single value filter; no value", () => {
    const url = new URL("https://example.com?key=search")
    expect(
        updateSearchTextFilter(url, "key", {
            filter: false,
        }).toString(),
    ).toEqual("https://example.com/")
})

test("update multiple value", () => {
    const url = new URL("https://example.com/")
    expect(
        updateSearchMultipleValueFilter(url, "key", ["search-1", "search-2"]).toString(),
    ).toEqual("https://example.com/?key=search-1&key=search-2")
})
test("update multiple value; overwrite", () => {
    const url = new URL("https://example.com/?key=original-1&key=original-2")
    expect(
        updateSearchMultipleValueFilter(url, "key", ["search-1", "search-2"]).toString(),
    ).toEqual("https://example.com/?key=search-1&key=search-2")
})

test("read search select result", () => {
    const url = new URL("https://example.com/?key=value")
    expect(readSearchSelectResult(url.searchParams, "key", (value) => value)).toEqual({
        isSelected: true,
        value: "value",
    })
})
test("read search select result; not selected", () => {
    const url = new URL("https://example.com/")
    expect(readSearchSelectResult(url.searchParams, "key", (value) => value)).toEqual({
        isSelected: false,
    })
})

test("update search select result", () => {
    const url = new URL("https://example.com/")
    expect(
        updateSearchSelectResult(url, "key", { isSelected: true, value: "value" }).toString(),
    ).toEqual("https://example.com/?key=value")
})
test("update search select result; not selected", () => {
    const url = new URL("https://example.com/?key=value")
    expect(updateSearchSelectResult(url, "key", { isSelected: false }).toString()).toEqual(
        "https://example.com/",
    )
})
