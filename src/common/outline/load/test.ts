import { test, expect } from "vitest"
import { observeApplicationState } from "../../../z_vendor/getto-application/action/test_helper"

import { markMenuCategoryLabel, standard_MenuTree } from "./test_helper"

import { initMemoryDB } from "../../util/repository/init/memory"
import { initMenuBadgeStore, initMenuExpandStore } from "./init/store"

import { detectMenuTargetPath } from "./convert"
import { convertMenuBadgeRemote, menuExpandRepositoryConverter } from "./convert"
import { authTicketRepositoryConverter } from "../../../auth/ticket/kernel/convert"
import { convertDB } from "../../util/repository/init/convert"

import { initOutlineMenuAction, OutlineMenuAction } from "./action"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../../../auth/ticket/kernel/infra"
import {
    MenuTargetPathDetecter,
    MenuExpandRepository,
    LoadMenuBadgeRemote,
    MenuExpandRepositoryValue,
} from "./infra"

import { AuthTicket } from "../../../auth/ticket/kernel/data"

test("load menu", async () => {
    const { menu } = standard()

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.state.ignitionState
        }),
    ).toEqual([
        {
            type: "succeed-to-load",
            menu: [
                $category("MAIN", ["MAIN"], 0, [
                    $item("ホーム", "home", "/1.0.0/index.html", 0),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 0),
                ]),
                category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
        {
            type: "succeed-to-update",
            menu: [
                $category("MAIN", ["MAIN"], 30, [
                    $item("ホーム", "home", "/1.0.0/index.html", 10),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 20),
                ]),
                category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
    ])
})

test("load menu; empty permissions", async () => {
    const { menu } = empty()

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.state.ignitionState
        }),
    ).toEqual([{ type: "required-to-login" }])
})

test("load menu; saved expands", async () => {
    const { menu } = expand()

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.state.ignitionState
        }),
    ).toEqual([
        {
            type: "succeed-to-load",
            menu: [
                $category("MAIN", ["MAIN"], 0, [
                    $item("ホーム", "home", "/1.0.0/index.html", 0),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 0),
                ]),
                $category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
        {
            type: "succeed-to-update",
            menu: [
                $category("MAIN", ["MAIN"], 30, [
                    $item("ホーム", "home", "/1.0.0/index.html", 10),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 20),
                ]),
                $category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
    ])
})

test("load menu; toggle expands", async () => {
    const { menu, repository } = standard()

    await menu.state.ignitionState

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.show([markMenuCategoryLabel("DOCUMENT")])
        }),
    ).toEqual([
        {
            type: "succeed-to-toggle",
            menu: [
                $category("MAIN", ["MAIN"], 30, [
                    $item("ホーム", "home", "/1.0.0/index.html", 10),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 20),
                ]),
                $category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
    ])

    expect(await repository.get()).toMatchObject({
        success: true,
        found: true,
        value: { values: [["DOCUMENT"]] },
    })

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.show([markMenuCategoryLabel("DOCUMENT"), markMenuCategoryLabel("DETAIL")])
        }),
    ).toEqual([
        {
            type: "succeed-to-toggle",
            menu: [
                $category("MAIN", ["MAIN"], 30, [
                    $item("ホーム", "home", "/1.0.0/index.html", 10),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 20),
                ]),
                $category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    $category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
    ])

    expect(await repository.get()).toMatchObject({
        success: true,
        found: true,
        value: { values: [["DOCUMENT"], ["DOCUMENT", "DETAIL"]] },
    })

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.hide([markMenuCategoryLabel("DOCUMENT"), markMenuCategoryLabel("DETAIL")])
        }),
    ).toEqual([
        {
            type: "succeed-to-toggle",
            menu: [
                $category("MAIN", ["MAIN"], 30, [
                    $item("ホーム", "home", "/1.0.0/index.html", 10),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 20),
                ]),
                $category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
            ],
        },
    ])

    expect(await repository.get()).toMatchObject({
        success: true,
        found: true,
        value: { values: [["DOCUMENT"]] },
    })
})

test("load menu; dev docs", async () => {
    const { menu } = user()

    expect(
        await observeApplicationState(menu.state, async () => {
            return menu.state.ignitionState
        }),
    ).toEqual([
        {
            type: "succeed-to-load",
            menu: [
                $category("MAIN", ["MAIN"], 0, [
                    $item("ホーム", "home", "/1.0.0/index.html", 0),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 0),
                ]),
                category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
                category("ACCOUNT", ["ACCOUNT"], 0, [
                    item("ユーザー", "friends", "/1.0.0/user/account.html", 0),
                ]),
            ],
        },
        {
            type: "succeed-to-update",
            menu: [
                $category("MAIN", ["MAIN"], 30, [
                    $item("ホーム", "home", "/1.0.0/index.html", 10),
                    item("ドキュメント", "docs", "/1.0.0/docs/index.html", 20),
                ]),
                category("DOCUMENT", ["DOCUMENT"], 0, [
                    item("認証・認可", "auth", "/1.0.0/docs/auth.html", 0),
                    category("DETAIL", ["DOCUMENT", "DETAIL"], 0, [
                        item("詳細", "detail", "/1.0.0/docs/auth.html", 0),
                    ]),
                ]),
                category("ACCOUNT", ["ACCOUNT"], 0, [
                    item("ユーザー", "friends", "/1.0.0/user/account.html", 0),
                ]),
            ],
        },
    ])
})

type MenuNode =
    | Readonly<{
          type: "category"
          category: Readonly<{ label: string }>
          path: string[]
          isExpand: boolean
          badgeCount: number
          children: MenuNode[]
      }>
    | Readonly<{
          type: "item"
          item: Readonly<{ label: string; icon: string; href: string }>
          isActive: boolean
          badgeCount: number
      }>

function category(label: string, path: string[], badgeCount: number, children: MenuNode[]) {
    return categoryNode(label, path, false, badgeCount, children)
}
function $category(label: string, path: string[], badgeCount: number, children: MenuNode[]) {
    return categoryNode(label, path, true, badgeCount, children)
}
function categoryNode(
    label: string,
    path: string[],
    isExpand: boolean,
    badgeCount: number,
    children: MenuNode[],
): MenuNode {
    return {
        type: "category",
        category: { label },
        path,
        isExpand,
        badgeCount,
        children,
    }
}

function item(label: string, icon: string, href: string, badgeCount: number) {
    return itemNode(label, icon, href, false, badgeCount)
}
function $item(label: string, icon: string, href: string, badgeCount: number) {
    return itemNode(label, icon, href, true, badgeCount)
}
function itemNode(
    label: string,
    icon: string,
    href: string,
    isActive: boolean,
    badgeCount: number,
): MenuNode {
    return {
        type: "item",
        item: { label, icon, href },
        isActive,
        badgeCount,
    }
}

function standard() {
    return initResource(standard_ticketRepository(), empty_menuExpandRepository())
}
function empty() {
    return initResource(empty_ticketRepository(), empty_menuExpandRepository())
}
function user() {
    return initResource(user_ticketRepository(), empty_menuExpandRepository())
}
function expand() {
    return initResource(standard_ticketRepository(), expand_menuExpandRepository())
}

function initResource(
    ticketRepository: AuthTicketRepository,
    menuExpandRepository: MenuExpandRepository,
): Readonly<{ menu: OutlineMenuAction; repository: MenuExpandRepository }> {
    const version = standard_version()
    const detectTargetPath = standard_detecter()
    const loadMenuBadgeRemote = standard_loadMenuBadgeRemote()

    return {
        menu: initOutlineMenuAction({
            infra: {
                loadMenuBadgeRemote: loadMenuBadgeRemote,
                ticketRepository,
                menuExpandRepository,
                menuExpandStore: initMenuExpandStore(),
                menuBadgeStore: initMenuBadgeStore(),
            },
            shell: {
                detectTargetPath,
            },
            config: {
                version,
                menuTree: standard_MenuTree(),
            },
        }),
        repository: menuExpandRepository,
    }
}

function standard_detecter(): MenuTargetPathDetecter {
    return () =>
        detectMenuTargetPath(new URL("https://example.com/1.0.0/index.html"), standard_version())
}
function standard_version(): string {
    return "1.0.0"
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        granted: ["admin"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}
function empty_ticketRepository(): AuthTicketRepository {
    return initMemoryDB<AuthTicket>()
}
function user_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        granted: ["auth-user"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}

function empty_menuExpandRepository(): MenuExpandRepository {
    return convertDB(initMemoryDB<MenuExpandRepositoryValue>(), menuExpandRepositoryConverter)
}
function expand_menuExpandRepository(): MenuExpandRepository {
    const db = initMemoryDB<MenuExpandRepositoryValue>()
    db.set([["DOCUMENT"]])
    return convertDB(db, menuExpandRepositoryConverter)
}

function standard_loadMenuBadgeRemote(): LoadMenuBadgeRemote {
    return async () => ({
        success: true,
        value: convertMenuBadgeRemote([
            { path: "index.html", count: 10 },
            { path: "docs/index.html", count: 20 },
        ]),
    })
}
