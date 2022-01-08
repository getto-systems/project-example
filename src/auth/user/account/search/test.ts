import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/init/mock"
import { mockSearchAuthUserAccountShell } from "./init/mock"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "./action"

import { BoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "./infra"

describe("SearchAuthUserAccount", () => {
    test("search", async () => {
        const { resource, store, url } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            store.loginID.set(markBoardValue("MY-LOGIN-ID"))
            resource.search.loginID.input.publisher.post()
            return resource.search.submit()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search" },
                {
                    type: "succeed-to-search",
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
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
                        users: [],
                    },
                },
            ])
        })
    })

    test("load", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            return resource.search.load()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search" },
                {
                    type: "succeed-to-search",
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        users: [],
                    },
                },
            ])
        })
    })

    test("sort", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            return resource.search.sort("login-id")
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search" },
                {
                    type: "succeed-to-search",
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        users: [],
                    },
                },
            ])
        })
    })

    test("ignite", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            resource.search.ignite()
            await ticker({ wait_millisecond: 0 }, () => null)
            return resource.search.currentState()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-search" },
                {
                    type: "succeed-to-search",
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
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

function initResource(searchRemote: SearchAuthUserAccountRemote): Readonly<{
    resource: Readonly<{ search: SearchAuthUserAccountAction }>
    store: Readonly<{
        loginID: BoardValueStore
    }>
    url: Readonly<{ current: URL }>
}> {
    const currentURL = new URL("https://example.com/index.html")

    const urlStore = { current: currentURL }

    const resource = {
        search: initSearchAuthUserAccountAction(
            {
                takeLongtimeThreshold: { delay_millisecond: 32 },
            },
            {
                searchRemote,
                columnsRepository: initMemoryDB(),
            },
            mockSearchAuthUserAccountShell(currentURL, (url) => {
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
        value: { page: { offset: 0, limit: 1000, all: 245 }, users: [] },
    }
}
