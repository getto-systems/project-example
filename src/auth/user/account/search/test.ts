import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { mockSearchAuthUserAccountShell } from "./init/mock"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "./action"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "./infra"
import { defaultSearchAuthUserAccountSort } from "./data"
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
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
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
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                },
                {
                    type: "succeed-to-search",
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
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
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                },
                {
                    type: "take-longtime-to-search",
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                },
                {
                    type: "succeed-to-search",
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
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
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                },
                {
                    type: "succeed-to-search",
                    previousResponse: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
                        users: [],
                    },
                    response: {
                        page: { offset: 0, limit: 1000, all: 245 },
                        sort: { key: "login-id", order: "normal" },
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
}> {
    const currentURL = new URL("https://example.com/index.html")

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

function standard_search(): SearchAuthUserAccountRemote {
    return async () => standard_searchRemoteResult()
}
function takeLongtime_search(): SearchAuthUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_searchRemoteResult())
}
function standard_searchRemoteResult(): SearchAuthUserAccountRemoteResult {
    return {
        success: true,
        value: {
            page: { offset: 0, limit: 1000, all: 245 },
            sort: { key: defaultSearchAuthUserAccountSort, order: "normal" },
            users: [],
        },
    }
}
