import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { mockSearchAuthUserAccountShell } from "./init/mock"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "./action"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "./infra"
import { defaultSearchAuthUserAccountSort, SearchAuthUserAccountRemoteResponse } from "./data"
import { readSearchAuthUserAccountSortKey } from "./convert"

describe("SearchAuthUserAccount", () => {
    test("initial load", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => resource.search.ignitionState).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search", previousResponse: undefined },
                {
                    type: "succeed-to-search",
                    previousResponse: undefined,
                    response: standard_response,
                },
            ])
        })
    })

    test("search", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await resource.search.ignitionState

        await runner(async () => {
            store.loginID.set(markBoardValue("MY-LOGIN-ID"))
            resource.search.loginID.input.publisher.post()
            return resource.search.search()
        }).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-search",
                    previousResponse: standard_response,
                },
                {
                    type: "succeed-to-search",
                    previousResponse: standard_response,
                    response: standard_response,
                },
            ])
        })
    })

    test("search; take longtime", async () => {
        const { resource } = takeLongtime()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await resource.search.ignitionState

        await runner(async () => {
            return resource.search.search()
        }).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-search",
                    previousResponse: standard_response,
                },
                {
                    type: "take-longtime-to-search",
                    previousResponse: standard_response,
                },
                {
                    type: "succeed-to-search",
                    previousResponse: standard_response,
                    response: standard_response,
                },
            ])
        })
    })

    test("sort", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await resource.search.ignitionState

        await runner(async () => {
            return resource.search.sort("login-id")
        }).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "try-to-search",
                    previousResponse: standard_response,
                },
                {
                    type: "succeed-to-search",
                    previousResponse: standard_response,
                    response: standard_response,
                },
            ])
        })
    })

    test("clear", () => {
        const { resource, store } = standard()

        store.loginID.set(markBoardValue("MY-LOGIN-ID"))
        resource.search.clear()

        expect(store.loginID.get()).toEqual("")
    })

    test("focus / close", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.search.detail.subscriber)

        await resource.search.ignitionState

        await runner(async () => {
            const user = { loginID: "user-1", grantedRoles: [] }
            const another = { loginID: "user-another", grantedRoles: [] }

            resource.search.detail.focus(user)
            expect(resource.search.detail.isFocused(user)).toBe(true)
            expect(resource.search.detail.isFocused(another)).toBe(false)

            resource.search.detail.close()
            expect(resource.search.detail.isFocused(user)).toBe(false)
            expect(resource.search.detail.isFocused(another)).toBe(false)

            return resource.search.detail.currentState()
        }).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "focus-on",
                    user: { loginID: "user-1", grantedRoles: [] },
                },
                { type: "initial-detail" },
            ])
        })
    })

    test("detect user", async () => {
        const { resource } = focused()

        const runner = setupActionTestRunner(resource.search.detail.subscriber)

        await runner(async () => {
            return resource.search.detail.ignitionState
        }).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "focus-detected",
                    user: { loginID: "user-1", grantedRoles: [] },
                },
            ])
        })
    })
    test("detect user; failed", async () => {
        const { resource } = focusFailed()

        const runner = setupActionTestRunner(resource.search.detail.subscriber)

        await runner(async () => {
            return resource.search.detail.ignitionState
        }).then((stack) => {
            expect(stack).toEqual([{ type: "focus-failed" }])
        })
    })

    test("terminate", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                resource.search.subscriber.subscribe(handler)
                resource.search.observe.subscriber.subscribe(handler)
                resource.search.loginID.observe.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            resource.search.terminate()
            return resource.search.search()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })

    test("read sort key", () => {
        expect(readSearchAuthUserAccountSortKey("login-id")).toEqual({
            found: true,
            key: "login-id",
        })
        expect(readSearchAuthUserAccountSortKey("unknown")).toEqual({
            found: false,
        })
    })
})

function standard() {
    return initResource(standard_url(), standard_search())
}
function takeLongtime() {
    return initResource(standard_url(), takeLongtime_search())
}
function focused() {
    return initResource(focused_url(), standard_search())
}
function focusFailed() {
    return initResource(focusFailed_url(), standard_search())
}

function initResource(
    currentURL: URL,
    searchRemote: SearchAuthUserAccountRemote,
): Readonly<{
    resource: Readonly<{ search: SearchAuthUserAccountAction }>
    store: Readonly<{
        loginID: BoardValueStore
    }>
}> {
    const urlStore = { current: currentURL }

    const resource = {
        search: initSearchAuthUserAccountAction({
            infra: {
                searchRemote,
                columnsRepository: initMemoryDB(),
            },
            shell: mockSearchAuthUserAccountShell(currentURL, (url) => {
                urlStore.current = url
            }),
            config: {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
        }),
    }

    const store = {
        loginID: mockBoardValueStore(),
    }

    resource.search.loginID.input.connector.connect(store.loginID)

    return { resource, store }
}

function standard_url(): URL {
    return new URL("https://example.com/index.html")
}
function focused_url(): URL {
    return new URL("https://example.com/index.html?id=user-1")
}
function focusFailed_url(): URL {
    return new URL("https://example.com/index.html?id=user-unknown")
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
    users: [
        { loginID: "user-1", grantedRoles: [] },
        { loginID: "user-2", grantedRoles: [] },
    ],
}
