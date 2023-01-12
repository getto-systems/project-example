import { staticMenuPath, StaticMenuPath } from "../../y_environment/ui/path"

import { MenuPermissionRequired, MenuTree, MenuTreeNode } from "../../common/outline/load/infra"

import { Icon } from "../../common/util/icon/data"

export function category(
    label: string,
    permission: MenuPermissionRequired,
    children: MenuTree,
): MenuTreeNode {
    return { type: "category", category: { label, required: permission }, children }
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
