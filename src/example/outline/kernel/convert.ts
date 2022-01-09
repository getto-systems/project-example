import { MenuTreeCategory, MenuTreeItem } from "./infra"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { MenuCategory, MenuCategoryLabel, MenuCategoryPath, MenuItem, MenuTargetPath } from "./data"

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

function markMenuTargetPath(target: string): MenuTargetPath {
    return target as MenuTargetPath
}
function markMenuCategoryLabel(label: string): MenuCategoryLabel {
    return label as MenuCategoryLabel
}
