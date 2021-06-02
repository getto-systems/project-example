import { appendMenuCategoryPath, toMenuCategory, toMenuItem } from "./converter"

import {
    MenuBadge,
    MenuExpand,
    MenuPermission,
    MenuTree,
    MenuTreeCategory,
    MenuTreeItem,
    MenuTreeNode,
} from "../infra"

import { ConvertLocationResult } from "../../../../../ui/vendor/getto-application/location/data"
import { GrantedRoles } from "../../../../auth/auth_ticket/_ui/kernel/data"
import { Menu, MenuCategoryPath, MenuNode, MenuTargetPath } from "../data"

export type BuildMenuParams = Readonly<{
    version: string
    menuTree: MenuTree
    menuTargetPath: ConvertLocationResult<MenuTargetPath>
    grantedRoles: GrantedRoles
    menuExpand: MenuExpand
    menuBadge: MenuBadge
}>
export function buildMenu(params: BuildMenuParams): Menu {
    const { version, menuTree, menuTargetPath, grantedRoles, menuExpand, menuBadge } = params

    return toMenu(menuTree, [])

    function toMenu(tree: MenuTree, categoryPath: MenuCategoryPath): Menu {
        return tree.flatMap((node) => toMenuNodes(node, categoryPath))
    }
    function toMenuNodes(node: MenuTreeNode, categoryPath: MenuCategoryPath): MenuNode[] {
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
    ): MenuNode[] {
        if (!isAllow(category.permission)) {
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

        function isAllow(permission: MenuPermission): boolean {
            switch (permission.type) {
                case "allow":
                    return true

                case "any":
                    return permission.permits.some(isAllow)

                case "all":
                    return permission.permits.every(isAllow)

                case "role":
                    return grantedRoles.includes(permission.role)
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

const EMPTY: MenuNode[] = []
