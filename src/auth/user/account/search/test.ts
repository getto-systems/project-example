import { test, expect } from "vitest"
import { observe2Atom, observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../../../common/util/timer/helper"

import {
    mockMultipleBoardStore,
    mockSingleBoardStore,
} from "../../../../common/util/board/input/test_helper"
import { mockSearchAuthUserAccountShell } from "./detail/mock"

import { readSearchAuthUserAccountSortKey } from "./detail/query"
import { restoreLoginId } from "../../login_id/kernel/convert"
import { restoreAuthUserField } from "../kernel/convert"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "./action"

import { MultipleBoardStore, SingleBoardStore } from "../../../../common/util/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "./infra"

import { defaultSearchAuthUserAccountSort, SearchAuthUserAccountRemoteResponse } from "./data"

test("initial load", async () => {
    const { search } = standard()

    const result = observeAtom(search.state)

    await search.state.ignitionState

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", response: standard_response },
    ])
})

test("search", async () => {
    const { search, store } = standard()

    await search.state.ignitionState

    const result = observeAtom(search.state)

    store.loginId.set("MY-LOGIN-ID")
    store.granted.set(["auth-user"])

    await search.search()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", response: standard_response },
    ])
})

test("search; take longtime", async () => {
    const { search } = takeLongtime()

    await search.state.ignitionState

    const result = observeAtom(search.state)

    await search.search()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "try", hasTakenLongtime: true },
        { type: "success", response: standard_response },
    ])
})

test("load", async () => {
    const { search } = standard()

    await search.state.ignitionState

    const result = observeAtom(search.state)

    await search.load()

    expect(result()).toEqual([
        { type: "try", hasTakenLongtime: false },
        { type: "success", response: standard_response },
    ])
})

test("sort", async () => {
    const { search } = standard()

    await search.state.ignitionState

    const result = observe2Atom(search.state, search.sortKey)

    await search.sort("loginId")

    expect(result()).toEqual([
        [
            { type: "success", response: standard_response },
            { key: "loginId", order: "reverse" },
        ],
        [
            { type: "try", hasTakenLongtime: false },
            { key: "loginId", order: "reverse" },
        ],
        [
            { type: "success", response: standard_response },
            { key: "loginId", order: "normal" },
        ],
        [
            { type: "success", response: standard_response },
            { key: "loginId", order: "normal" },
        ],
    ])
})

test("reset", () => {
    const { search, store } = standard()

    store.loginId.set("MY-LOGIN-ID")
    search.reset()

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

function standard() {
    return initResource(standard_url(), standard_search())
}
function takeLongtime() {
    return initResource(standard_url(), takeLongtime_search())
}

function initResource(
    currentURL: URL,
    searchRemote: SearchAuthUserAccountRemote,
): Readonly<{
    search: SearchAuthUserAccountAction
    store: Readonly<{
        loginId: SingleBoardStore
        granted: MultipleBoardStore
    }>
}> {
    const urlStore = { current: currentURL }

    const [search, _updater] = initSearchAuthUserAccountAction({
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
        loginId: mockSingleBoardStore(search.loginId.input),
        granted: mockMultipleBoardStore(search.granted.input),
    }

    return { search, store }
}

function standard_url(): URL {
    return new URL("https://example.com/index.html?filter-granted=auth-user")
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
    page: { offset: 0, limit: 1000, count: 245 },
    sort: { key: defaultSearchAuthUserAccountSort, order: "normal" },
    list: [
        {
            loginId: restoreLoginId("user-1"),
            granted: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserField("memo"),
        },
        {
            loginId: restoreLoginId("user-2"),
            granted: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserField("memo"),
        },
    ],
}
