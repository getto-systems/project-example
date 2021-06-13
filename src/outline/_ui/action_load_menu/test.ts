import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"

import { markMenuCategoryLabel, standard_MenuTree } from "../kernel/test_helper"

import { convertRepository } from "../../../z_details/_ui/repository/helper"
import { mockRemotePod } from "../../../z_details/_ui/remote/mock"
import { mockRepository } from "../../../z_details/_ui/repository/mock"

import { mockLoadMenuLocationDetecter } from "../kernel/mock"

import { initLoadMenuCoreAction, initLoadMenuCoreMaterial } from "./core/impl"

import { menuExpandRepositoryConverter } from "../kernel/converter"

import {
    AuthzRepositoryPod,
    AuthzRepositoryValue,
} from "../../../auth/auth_ticket/_ui/kernel/infra"
import {
    GetMenuBadgeRemotePod,
    MenuExpandRepositoryPod,
    MenuExpandRepositoryValue,
} from "../kernel/infra"

import { LoadMenuResource } from "./resource"

import { LoadMenuDetecter } from "../kernel/method"

describe("Menu", () => {
    test("load menu", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignite()).then((stack) => {
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

        await runner(() => resource.menu.ignite()).then((stack) => {
            expect(stack).toEqual([{ type: "required-to-login" }])
        })
    })

    test("load menu; saved expands", async () => {
        const { resource } = expand()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignite()).then((stack) => {
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
        const { resource, repository } = standard()
        const menuExpand = repository.menuExpand(menuExpandRepositoryConverter)

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignite())
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
        const { resource } = devDocs()

        const runner = setupActionTestRunner(resource.menu.subscriber)

        await runner(() => resource.menu.ignite()).then((stack) => {
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
                        category("DEVELOPMENT", ["DEVELOPMENT"], 0, [
                            item("配備構成", "deployment", "/1.0.0/docs/z-dev/deployment.html", 0),
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
                        category("DEVELOPMENT", ["DEVELOPMENT"], 0, [
                            item("配備構成", "deployment", "/1.0.0/docs/z-dev/deployment.html", 0),
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
            return resource.menu.ignite()
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
    const [resource, menuExpand] = initResource(standard_authz(), empty_menuExpand())

    return { resource, repository: { menuExpand } }
}
function empty() {
    const [resource] = initResource(empty_authz(), empty_menuExpand())

    return { resource }
}
function devDocs() {
    const [resource] = initResource(devDocs_authz(), empty_menuExpand())

    return { resource }
}
function expand() {
    const [resource] = initResource(standard_authz(), expand_menuExpand())

    return { resource }
}

function initResource(
    authz: AuthzRepositoryPod,
    menuExpand: MenuExpandRepositoryPod,
): [LoadMenuResource, MenuExpandRepositoryPod] {
    const version = standard_version()
    const detecter = standard_detecter()
    const getMenuBadge = standard_getMenuBadge()

    return [
        {
            menu: initLoadMenuCoreAction(
                initLoadMenuCoreMaterial(
                    {
                        version,
                        menuTree: standard_MenuTree(),
                        authz,
                        menuExpand,
                        getMenuBadge,
                    },
                    detecter,
                ),
            ),
        },
        menuExpand,
    ]
}

function standard_detecter(): LoadMenuDetecter {
    return mockLoadMenuLocationDetecter(
        new URL("https://example.com/1.0.0/index.html"),
        standard_version(),
    )
}
function standard_version(): string {
    return "1.0.0"
}

function standard_authz(): AuthzRepositoryPod {
    const authz = mockRepository<AuthzRepositoryValue>()
    authz.set({ roles: ["admin"] })
    return convertRepository(authz)
}
function empty_authz(): AuthzRepositoryPod {
    return convertRepository(mockRepository<AuthzRepositoryValue>())
}
function devDocs_authz(): AuthzRepositoryPod {
    const authz = mockRepository<AuthzRepositoryValue>()
    authz.set({ roles: ["admin", "dev-docs"] })
    return convertRepository(authz)
}

function empty_menuExpand(): MenuExpandRepositoryPod {
    return convertRepository(mockRepository<MenuExpandRepositoryValue>())
}
function expand_menuExpand(): MenuExpandRepositoryPod {
    const menuExpand = mockRepository<MenuExpandRepositoryValue>()
    menuExpand.set([["DOCUMENT"]])
    return convertRepository(menuExpand)
}

function standard_getMenuBadge(): GetMenuBadgeRemotePod {
    return mockRemotePod(
        () => ({
            success: true,
            value: [
                { path: "/index.html", count: 10 },
                { path: "/docs/index.html", count: 20 },
            ],
        }),
        { wait_millisecond: 0 },
    )
}
