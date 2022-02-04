import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"
import {
    initMenuExpand,
    MenuBadge,
    MenuBadgeItem,
    MenuExpand,
    MenuTreeCategory,
    MenuTreeItem,
    MenuExpandRepositoryValue,
} from "./infra"

import { MenuCategoryLabel, MenuCategory, MenuCategoryPath, MenuItem, MenuTargetPath } from "./data"
import { ConvertLocationResult } from "../../../z_lib/ui/location/data"

export function detectMenuTargetPath(
    currentURL: URL,
    version: string,
): ConvertLocationResult<MenuTargetPath> {
    const pathname = currentURL.pathname
    const versionPrefix = `/${version}/`
    if (!pathname.startsWith(versionPrefix)) {
        return { valid: false }
    }
    return {
        valid: true,
        value: markMenuTargetPath(pathname.replace(versionPrefix, "")),
    }
}

export function toMenuCategory(category: MenuTreeCategory): MenuCategory {
    return {
        label: markMenuCategoryLabel(category.label),
    }
}
export function appendMenuCategoryPath(
    path: MenuCategoryPath,
    category: MenuTreeCategory,
): MenuCategoryPath {
    return [...path, markMenuCategoryLabel(category.label)]
}
export function toMenuItem({ label, icon, path }: MenuTreeItem, version: string): MenuItem {
    return { label, icon, href: `/${version}/${path}` } as MenuItem
}

export function convertMenuBadgeRemote(menuBadgeItems: readonly MenuBadgeItem[]): MenuBadge {
    return menuBadgeItems.reduce((acc, item) => {
        acc.set(item.path, item.count)
        return acc
    }, <MenuBadge>new Map())
}

export const menuExpandRepositoryConverter: RepositoryConverter<
    MenuExpand,
    MenuExpandRepositoryValue
> = {
    toRepository: (value) => value.values,
    fromRepository: (value) => {
        // label の配列なので、validation error にする手がかりがない
        const menuExpand = initMenuExpand()
        menuExpand.init(value.map((labels) => labels.map(markMenuCategoryLabel)))

        return {
            valid: true,
            value: menuExpand,
        }
    },
}

function markMenuTargetPath(target: string): MenuTargetPath {
    return target as MenuTargetPath
}
function markMenuCategoryLabel(label: string): MenuCategoryLabel {
    return label as MenuCategoryLabel
}
