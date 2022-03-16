import { setupActionTestRunner } from "../../../z_vendor/getto-application/action/test_helper"

import { markMenuCategoryLabel, standard_MenuTree } from "./test_helper"

import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"
import { initMenuBadgeStore, initMenuExpandStore } from "./init/store"

import { detectMenuTargetPath } from "./convert"
import { convertMenuBadgeRemote, menuExpandRepositoryConverter } from "./convert"
import { authTicketRepositoryConverter } from "../../../auth/ticket/kernel/convert"
import { convertDB } from "../../../z_lib/ui/repository/init/convert"

import { initLoadMenuAction, LoadMenuAction } from "./action"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../../../auth/ticket/kernel/infra"
import {
    MenuTargetPathDetecter,
    MenuExpandRepository,
    LoadMenuBadgeRemote,
    MenuExpandRepositoryValue,
} from "./infra"

import { AuthTicket } from "../../../auth/ticket/kernel/data"

describe("Menu", () => {
    test("load menu", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignitionState).then((stack) => {
            expect(stack).toEqual([
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
    })

    test("load menu; empty roles", async () => {
        const { resource } = empty()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignitionState).then((stack) => {
            expect(stack).toEqual([{ type: "required-to-login" }])
        })
    })

    test("load menu; saved expands", async () => {
        const { resource } = expand()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignitionState).then((stack) => {
            expect(stack).toEqual([
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
    })

    test("load menu; toggle expands", async () => {
        const { resource, menuExpand } = standard()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignitionState)
        await runner(() => resource.menu.show([markMenuCategoryLabel("DOCUMENT")])).then(
            (stack) => {
                expect(stack).toEqual([
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
            },
        )
        await runner(() =>
            resource.menu.show([
                markMenuCategoryLabel("DOCUMENT"),
                markMenuCategoryLabel("DETAIL"),
            ]),
        ).then(async (stack) => {
            expect(stack).toEqual([
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

            const result = await menuExpand.get()
            if (!result.success) {
                throw new Error("menu expand get failed")
            }
            if (!result.found) {
                throw new Error("menu expand not found")
            }
            expect(result.value.values).toEqual([["DOCUMENT"], ["DOCUMENT", "DETAIL"]])
        })
        await runner(() =>
            resource.menu.hide([
                markMenuCategoryLabel("DOCUMENT"),
                markMenuCategoryLabel("DETAIL"),
            ]),
        ).then(async (stack) => {
            expect(stack).toEqual([
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

            const result = await menuExpand.get()
            if (!result.success) {
                throw new Error("menu expand get failed")
            }
            if (!result.found) {
                throw new Error("menu expand not found")
            }
            expect(result.value.values).toEqual([["DOCUMENT"]])
        })
    })

    test("load menu; dev docs", async () => {
        const { resource } = user()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignitionState).then((stack) => {
            expect(stack).toEqual([
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
    })

    test("terminate", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => {
            resource.menu.terminate()
            return resource.menu.ignitionState
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
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
})

function standard() {
    const [resource, menuExpand] = initResource(
        standard_ticketRepository(),
        empty_menuExpandRepository(),
    )

    return { resource, menuExpand }
}
function empty() {
    const [resource] = initResource(empty_ticketRepository(), empty_menuExpandRepository())

    return { resource }
}
function user() {
    const [resource] = initResource(user_authz(), empty_menuExpandRepository())

    return { resource }
}
function expand() {
    const [resource] = initResource(standard_ticketRepository(), expand_menuExpandRepository())

    return { resource }
}

function initResource(
    ticketRepository: AuthTicketRepository,
    menuExpandRepository: MenuExpandRepository,
): [Readonly<{ menu: LoadMenuAction }>, MenuExpandRepository] {
    const version = standard_version()
    const detectTargetPath = standard_detecter()
    const loadMenuBadgeRemote = standard_loadMenuBadgeRemote()

    return [
        {
            menu: initLoadMenuAction({
                infra: {
                    loadMenuBadgeRemote,
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
        },
        menuExpandRepository,
    ]
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
        grantedRoles: ["admin"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}
function empty_ticketRepository(): AuthTicketRepository {
    return initMemoryDB<AuthTicket>()
}
function user_authz(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 00:00:00",
        grantedRoles: ["user"],
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
