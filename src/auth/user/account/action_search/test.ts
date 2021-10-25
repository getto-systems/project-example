import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockSearchColumnsRepository } from "../../../../z_lib/ui/search/columns/init/repository/mock"
import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/init/mock"
import {
    mockSearchAuthUserAccountFieldsDetecter,
    mockUpdateSearchAuthUserAccountFieldsQuery,
} from "../search/mock"

import { initSearchAuthUserAccountAction, initSearchAuthUserAccountMaterial } from "./init"

import { BoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "../search/infra"

import { SearchAuthUserAccountResource } from "./resource"

describe("SearchAuthUserAccount", () => {
    test("search", async () => {
        const { resource, store, url } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            store.loginID.set(markBoardValue("MY-LOGIN-ID"))
            return resource.search.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search" },
                {
                    type: "succeed-to-search",
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        summary: {},
                        users: [],
                    },
                },
            ])
            expect(url.current.toString()).toEqual(
                "https://example.com/index.html?login-id=MY-LOGIN-ID&search-offset=0&search-sort-key=login-id&search-sort-order=normal",
            )
        })
    })

    test("search; take longtime", async () => {
        const { resource } = takeLongtime()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            return resource.search.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search" },
                { type: "take-longtime-to-search" },
                {
                    type: "succeed-to-search",
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        summary: {},
                        users: [],
                    },
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
            return resource.search.submit()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    return initResource(standard_search())
}
function takeLongtime() {
    return initResource(takeLongtime_search())
}

function initResource(search: SearchAuthUserAccountRemote): Readonly<{
    resource: SearchAuthUserAccountResource
    store: Readonly<{
        loginID: BoardValueStore
    }>
    url: Readonly<{ current: URL }>
}> {
    const currentURL = new URL("https://example.com/index.html")

    const urlStore = { current: currentURL }

    const resource = {
        search: initSearchAuthUserAccountAction(
            initSearchAuthUserAccountMaterial({
                search: {
                    search,
                    config: {
                        takeLongtimeThreshold: { delay_millisecond: 32 },
                    },
                },
                columns: {
                    columns: mockSearchColumnsRepository(),
                },
            }),
            mockSearchAuthUserAccountFieldsDetecter(currentURL),
            mockUpdateSearchAuthUserAccountFieldsQuery(currentURL, (url) => {
                urlStore.current = url
            }),
        ),
    }

    const store = {
        loginID: mockBoardValueStore(),
    }

    resource.search.loginID.input.connector.connect(store.loginID)

    return { resource, store, url: urlStore }
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
        value: { page: { offset: 0, limit: 1000, all: 245 }, summary: {}, users: [] },
    }
}
