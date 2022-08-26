import { test, expect } from "vitest"
import {
    observeApplicationState,
    observeApplicationStateTuple2,
} from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { mockSearchAuthUserAccountShell } from "./init/mock"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "./action"

import { readSearchAuthUserAccountSortKey } from "./convert"
import { restoreLoginId } from "../../login_id/input/convert"
import { restoreAuthUserField } from "../kernel/convert"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "./infra"

import { defaultSearchAuthUserAccountSort, SearchAuthUserAccountRemoteResponse } from "./data"
import { AuthUserAccount } from "../kernel/data"

test("initial load", async () => {
    const { search } = standard()

    expect(
        await observeApplicationState(search.state, async () => {
            return search.state.ignitionState
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", response: standard_response },
    ])
})

test("search", async () => {
    const { search, store } = standard()

    await search.state.ignitionState

    expect(
        await observeApplicationState(search.state, async () => {
            store.loginId.set("MY-LOGIN-ID")
            return search.search()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", response: standard_response },
    ])
})

test("search; take longtime", async () => {
    const { search } = takeLongtime()

    await search.state.ignitionState

    expect(
        await observeApplicationState(search.state, async () => {
            return search.search()
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        { type: "success", response: standard_response },
    ])
})

test("sort", async () => {
    const { search } = standard()

    await search.state.ignitionState

    expect(
        await observeApplicationState(search.state, async () => {
            return search.sort("loginId")
        }),
    ).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", response: standard_response },
    ])
    expect(search.currentSort()).toEqual({ key: "loginId", order: "normal" })
})

test("clear", () => {
    const { search, store } = standard()

    store.loginId.set("MY-LOGIN-ID")
    search.clear()

    expect(store.loginId.get()).toEqual("")
})

test("read sort key", () => {
    expect(readSearchAuthUserAccountSortKey("loginId")).toEqual({
        found: true,
        key: "loginId",
    })
    expect(readSearchAuthUserAccountSortKey("unknown")).toEqual({
        found: false,
    })
})

test("detected", async () => {
    const { search } = detected()

    expect(
        await observeApplicationStateTuple2(
            [search.list.focus.state, search.list.scroll.state],
            async () => {
                await search.list.state.ignitionState
                return search.list.focus.state.currentState()
            },
        ),
    ).toEqual([[{ type: "focus-change", data: standard_response.list[0] }], [{ type: "detect" }]])
})

test("focus / close", async () => {
    const { search } = standard()

    expect(
        await observeApplicationState(search.list.focus.state, async () => {
            await search.list.state.ignitionState
            const another: AuthUserAccount = {
                loginId: restoreLoginId("another-1"),
                grantedRoles: [],
                resetTokenDestination: { type: "none" },
                memo: restoreAuthUserField("memo"),
            }

            search.list.focus.change(standard_response.list[0], { y: 0 })
            expect(search.list.focus.isFocused(standard_response.list[0])).toBe(true)
            expect(search.list.focus.isFocused(another)).toBe(false)

            search.list.focus.close({ y: 0 })
            expect(search.list.focus.isFocused(standard_response.list[0])).toBe(false)
            expect(search.list.focus.isFocused(another)).toBe(false)

            search.list.focus.change(another, { y: 0 })

            return search.list.focus.state.currentState()
        }),
    ).toEqual([
        { type: "focus-change", data: standard_response.list[0] },
        { type: "close" },
        { type: "not-found" },
    ])
})

function standard() {
    return initResource(standard_url(), standard_search())
}
function takeLongtime() {
    return initResource(standard_url(), takeLongtime_search())
}
function detected() {
    return initResource(detected_url(), standard_search())
}

function initResource(
    currentURL: URL,
    searchRemote: SearchAuthUserAccountRemote,
): Readonly<{
    search: SearchAuthUserAccountAction
    store: Readonly<{
        loginId: BoardValueStore
    }>
}> {
    const urlStore = { current: currentURL }

    const search = initSearchAuthUserAccountAction({
        infra: {
            searchRemote,
        },
        shell: mockSearchAuthUserAccountShell(currentURL, (url) => {
            urlStore.current = url
        }),
        config: {
            takeLongtimeThreshold: { wait_millisecond: 32 },
        },
    })

    const store = {
        loginId: mockBoardValueStore(search.loginId.input),
    }

    return { search, store }
}

function standard_url(): URL {
    return new URL("https://example.com/index.html")
}
function detected_url(): URL {
    return new URL("https://example.com/index.html?id=user-1")
}

function standard_search(): SearchAuthUserAccountRemote {
    return async () => standard_searchRemoteResult()
}
function takeLongtime_search(): SearchAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_searchRemoteResult())
}
function standard_searchRemoteResult(): SearchAuthUserAccountRemoteResult {
    return {
        success: true,
        value: standard_response,
    }
}

const standard_response: SearchAuthUserAccountRemoteResponse = {
    page: { offset: 0, limit: 1000, all: 245 },
    sort: { key: defaultSearchAuthUserAccountSort, order: "normal" },
    list: [
        {
            loginId: restoreLoginId("user-1"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserField("memo"),
        },
        {
            loginId: restoreLoginId("user-2"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserField("memo"),
        },
    ],
}
