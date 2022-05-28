import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"
import { ticker } from "../../../../z_lib/ui/timer/helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { mockSearchAuthUserAccountShell } from "./init/mock"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "./action"

import { readSearchAuthUserAccountSortKey } from "./convert"
import { restoreLoginId } from "../../login_id/input/convert"
import { restoreResetTokenDestination } from "../../password/reset/token_destination/kernel/convert"
import { restoreAuthUserMemo } from "../input/memo/convert"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "./infra"

import { defaultSearchAuthUserAccountSort, SearchAuthUserAccountRemoteResponse } from "./data"
import { AuthUserAccount } from "../kernel/data"

test("initial load", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner(resource.search.subscriber)

    await runner(async () => resource.search.ignitionState).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false, previousResponse: undefined },
            { type: "success", response: standard_response },
        ])
    })
})

test("search", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.search.subscriber)

    await resource.search.ignitionState

    await runner(async () => {
        store.loginId.set("MY-LOGIN-ID")
        return resource.search.search()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "try", hasTakenLongtime: false, previousResponse: standard_response },
            { type: "success", response: standard_response },
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
            { type: "try", hasTakenLongtime: false, previousResponse: standard_response },
            { type: "try", hasTakenLongtime: true, previousResponse: standard_response },
            { type: "success", response: standard_response },
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
            { type: "try", hasTakenLongtime: false, previousResponse: standard_response },
            { type: "success", response: standard_response },
        ])
        expect(resource.search.currentSort()).toEqual({ key: "login-id", order: "normal" })
    })
})

test("clear", () => {
    const { resource, store } = standard()

    store.loginId.set("MY-LOGIN-ID")
    resource.search.clear()

    expect(store.loginId.get()).toEqual("")
})

test("focus / close", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner(resource.search.focused.subscriber)

    await resource.search.ignitionState

    await runner(async () => {
        const user: AuthUserAccount = {
            loginId: restoreLoginId("user-1"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserMemo("memo"),
        }
        const another: AuthUserAccount = {
            loginId: restoreLoginId("user-another"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserMemo("memo"),
        }

        resource.search.focused.focus(user)
        expect(resource.search.focused.isFocused(user)).toBe(true)
        expect(resource.search.focused.isFocused(another)).toBe(false)

        resource.search.focused.close()
        expect(resource.search.focused.isFocused(user)).toBe(false)
        expect(resource.search.focused.isFocused(another)).toBe(false)

        return resource.search.focused.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "focus-on",
                user: {
                    loginId: "user-1",
                    grantedRoles: [],
                    resetTokenDestination: { type: "none" },
                    memo: "memo",
                },
            },
            { type: "initial" },
        ])
    })
})

test("detect user", async () => {
    const { resource } = focused()

    const runner = setupActionTestRunner(resource.search.focused.subscriber)

    await runner(async () => {
        return resource.search.focused.ignitionState
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "focus-detected",
                user: {
                    loginId: "user-1",
                    grantedRoles: [],
                    resetTokenDestination: { type: "none" },
                    memo: "memo",
                },
            },
        ])
    })
})
test("detect user; failed", async () => {
    const { resource } = focusFailed()

    const runner = setupActionTestRunner(resource.search.focused.subscriber)

    await runner(async () => {
        return resource.search.focused.ignitionState
    }).then((stack) => {
        expect(stack).toEqual([{ type: "focus-failed" }])
    })
})

test("update user", async () => {
    const { resource } = focused()

    const runner = setupActionTestRunner(resource.search.focused.subscriber)

    await resource.search.ignitionState

    const user: AuthUserAccount = {
        loginId: restoreLoginId("user-1"),
        grantedRoles: ["auth-user"],
        resetTokenDestination: restoreResetTokenDestination({
            type: "email",
            email: "user@example.com",
        }),
        memo: restoreAuthUserMemo("memo"),
    }

    await runner(async () => {
        return resource.search.focused.update(user.loginId, user)
    }).then((stack) => {
        expect(stack).toEqual([{ type: "focus-on", user }])
    })
})

test("remove user", async () => {
    const { resource } = focused()

    const runner = setupActionTestRunner(resource.search.focused.subscriber)

    await resource.search.ignitionState

    await runner(async () => {
        return resource.search.focused.remove(restoreLoginId("user-1"))
    }).then((stack) => {
        expect(stack).toEqual([{ type: "initial" }])
    })
})

test("terminate", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.search.subscriber.subscribe(handler)
            resource.search.observe.subscriber.subscribe(handler)
            resource.search.loginId.observe.subscriber.subscribe(handler)
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
        loginId: BoardValueStore
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
                takeLongtimeThreshold: { wait_millisecond: 32 },
            },
        }),
    }

    const store = {
        loginId: mockBoardValueStore(resource.search.loginId.input),
    }

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
        {
            loginId: restoreLoginId("user-1"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserMemo("memo"),
        },
        {
            loginId: restoreLoginId("user-2"),
            grantedRoles: [],
            resetTokenDestination: { type: "none" },
            memo: restoreAuthUserMemo("memo"),
        },
    ],
}
