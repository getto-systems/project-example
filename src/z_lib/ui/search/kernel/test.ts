import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import {
    readMultipleValueFilter,
    readSingleValueFilter,
    updateMultipleValueFilter,
    updateSingleValueFilter,
} from "./convert"

describe("filter", () => {
    test("read single value filter", () => {
        const url = new URL("https://example.com/?key=search")
        expect(readSingleValueFilter(url.searchParams, "key")).toEqual({
            search: true,
            value: "search",
        })
    })
    test("read single value filter; not found", () => {
        const url = new URL("https://example.com/")
        expect(readSingleValueFilter(url.searchParams, "key")).toEqual({
            search: false,
        })
    })

    test("read multiple value filter", () => {
        const url = new URL("https://example.com/?key=search-1&key=search-2")
        expect(readMultipleValueFilter(url.searchParams, "key")).toEqual([
            "search-1",
            "search-2",
        ])
    })
    test("read multiple value filter; not found", () => {
        const url = new URL("https://example.com/")
        expect(readMultipleValueFilter(url.searchParams, "key")).toEqual([])
    })

    test("update single value filter", () => {
        const url = new URL("https://example.com/")
        expect(
            updateSingleValueFilter(url, "key", {
                search: true,
                value: markBoardValue("search"),
            }).toString(),
        ).toEqual("https://example.com/?key=search")
    })
    test("update single value filter; no value", () => {
        const url = new URL("https://example.com?key=search")
        expect(
            updateSingleValueFilter(url, "key", {
                search: false,
            }).toString(),
        ).toEqual("https://example.com/")
    })

    test("update multiple value", () => {
        const url = new URL("https://example.com/")
        expect(
            updateMultipleValueFilter(url, "key", [
                markBoardValue("search-1"),
                markBoardValue("search-2"),
            ]).toString(),
        ).toEqual("https://example.com/?key=search-1&key=search-2")
    })
    test("update multiple value; override", () => {
        const url = new URL("https://example.com/?key=original-1&key=original-2")
        expect(
            updateMultipleValueFilter(url, "key", [
                markBoardValue("search-1"),
                markBoardValue("search-2"),
            ]).toString(),
        ).toEqual("https://example.com/?key=search-1&key=search-2")
    })
})