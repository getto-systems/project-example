import { RepositoryConverter } from "../../../z_details/_ui/repository/infra"
import {
    initMenuExpand,
    MenuBadge,
    MenuBadgeItem,
    MenuExpand,
    MenuExpandRepositoryValue,
    MenuTreeCategory,
    MenuTreeItem,
} from "./infra"

import { ConvertLocationResult } from "../../../z_details/_ui/location/data"
import {
    MenuCategory,
    MenuCategoryLabel,
    MenuCategoryPath,
    MenuItem,
    MenuTargetPath,
} from "./data"

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
        value: markMenuTargetPath(pathname.replace(versionPrefix, "/")),
    }
}

export function menuBadgeRemoteConverter(menuBadgeItems: MenuBadgeItem[]): MenuBadge {
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
    return { label, icon, href: `/${version}${path}` } as MenuItem
}

function markMenuTargetPath(target: string): MenuTargetPath {
    return target as MenuTargetPath
}
function markMenuCategoryLabel(label: string): MenuCategoryLabel {
    return label as MenuCategoryLabel
}
