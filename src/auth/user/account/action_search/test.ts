import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/init/mock"
import {
    mockSearchUserAccountFieldsDetecter,
    mockUpdateSearchUserAccountFieldsQuery,
} from "../search/mock"

import { initSearchUserAccountAction, initSearchUserAccountMaterial } from "./init"

import { BoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"
import { SearchUserAccountRemote, SearchUserAccountRemoteResult } from "../search/infra"

import { SearchUserAccountResource } from "./resource"

describe("SearchUserAccount", () => {
    test("search", async () => {
        const { resource, store, url } = standard()

        const runner = setupActionTestRunner(resource.search.subscriber)

        await runner(async () => {
            store.loginID.set(markBoardValue("MY-LOGIN-ID"))
            return resource.search.submit()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "try-to-search" }, { type: "succeed-to-search" }])
            expect(url.current.toString()).toEqual(
                "https://example.com/index.html?login-id=MY-LOGIN-ID",
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
                { type: "succeed-to-search" },
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

function initResource(search: SearchUserAccountRemote): Readonly<{
    resource: SearchUserAccountResource
    store: Readonly<{
        loginID: BoardValueStore
    }>
    url: Readonly<{ current: URL }>
}> {
    const currentURL = new URL("https://example.com/index.html")

    const urlStore = { current: currentURL }

    const resource = {
        search: initSearchUserAccountAction(
            initSearchUserAccountMaterial({
                search: {
                    search,
                    config: {
                        takeLongtimeThreshold: { delay_millisecond: 32 },
                    },
                },
            }),
            mockSearchUserAccountFieldsDetecter(currentURL),
            mockUpdateSearchUserAccountFieldsQuery(currentURL, (url) => {
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

function standard_search(): SearchUserAccountRemote {
    return async () => standard_searchRemoteResult()
}
function takeLongtime_search(): SearchUserAccountRemote {
    return async () => ticker({ wait_millisecond: 64 }, () => standard_searchRemoteResult())
}
function standard_searchRemoteResult(): SearchUserAccountRemoteResult {
    return {
        success: true,
        value: { page: { offset: 0, limit: 1000, all: 245 }, users: [] },
    }
}
