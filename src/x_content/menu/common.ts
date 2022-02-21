import { staticMenuPath, StaticMenuPath } from "../../y_environment/ui/path"

import { MenuPermission, MenuTree, MenuTreeNode } from "../../core/outline/load/infra"

import { Icon } from "../../z_lib/ui/icon/data"

export function category(
    label: string,
    permission: MenuPermission,
    children: MenuTree,
): MenuTreeNode {
    return { type: "category", category: { label, permission }, children }
}

export function item(label: string, icon: Icon, path: StaticMenuPath): MenuTreeNode {
    return { type: "item", item: { label, icon, path } }
}

export function assertMenuPath(path: string): StaticMenuPath {
    if (staticMenuPath.filter((staticPath) => staticPath === path).length === 0) {
        throw new Error(`path is not included in static-path: ${path}`)
    }
    return path as StaticMenuPath
}
