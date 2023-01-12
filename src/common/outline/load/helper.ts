import { appendMenuCategoryPath, toMenuCategory, toMenuItem } from "./convert"

import {
    MenuBadge,
    MenuExpand,
    MenuPermissionRequired,
    MenuTree,
    MenuTreeCategory,
    MenuTreeItem,
    MenuTreeNode,
} from "./infra"

import { ConvertLocationResult } from "../../util/location/data"
import { AuthTicket } from "../../../auth/ticket/kernel/data"
import { Menu, MenuCategoryPath, MenuNode, MenuTargetPath } from "./data"

export type BuildMenuParams = Readonly<{
    version: string
    menuTree: MenuTree
    menuTargetPath: ConvertLocationResult<MenuTargetPath>
    ticket: AuthTicket
    menuExpand: MenuExpand
    menuBadge: MenuBadge
}>
export function buildMenu(params: BuildMenuParams): Menu {
    const { version, menuTree, menuTargetPath, ticket, menuExpand, menuBadge } = params

    return toMenu(menuTree, [])

    function toMenu(tree: MenuTree, categoryPath: MenuCategoryPath): Menu {
        return tree.flatMap((node) => toMenuNodes(node, categoryPath))
    }
    function toMenuNodes(node: MenuTreeNode, categoryPath: MenuCategoryPath): readonly MenuNode[] {
        switch (node.type) {
            case "item":
                return [itemNode(node.item)]

            case "category":
                return categoryNode(
                    node.category,
                    node.children,
                    appendMenuCategoryPath(categoryPath, node.category),
                )
        }
    }

    function itemNode(item: MenuTreeItem): MenuNode {
        return {
            type: "item",
            isActive: menuTargetPath.valid ? item.path === menuTargetPath.value : false,
            badgeCount: menuBadge.get(item.path) || 0,
            item: toMenuItem(item, version),
        }
    }

    function categoryNode(
        category: MenuTreeCategory,
        menuTree: MenuTree,
        path: MenuCategoryPath,
    ): readonly MenuNode[] {
        if (!isAllow(category.required)) {
            return EMPTY
        }

        const children = toMenu(menuTree, path)
        if (children.length === 0) {
            return EMPTY
        }

        return [
            {
                type: "category",
                isExpand: menuExpand.hasEntry(path) || children.some(hasActive),
                badgeCount: children.reduce((acc, node) => acc + node.badgeCount, 0),
                category: toMenuCategory(category),
                children,
                path,
            },
        ]

        function isAllow(required: MenuPermissionRequired): boolean {
            switch (required.type) {
                case "nothing":
                    return true

                case "has-some":
                    for (const permission of required.permissions) {
                        if (ticket.granted.includes(permission)) {
                            return true
                        }
                    }
                    return false
            }
        }
        function hasActive(node: MenuNode): boolean {
            switch (node.type) {
                case "category":
                    return node.children.some(hasActive)
                case "item":
                    return node.isActive
            }
        }
    }
}

const EMPTY: readonly MenuNode[] = []
