import { staticMenuPath, StaticMenuPath } from "../../../../y_environment/_ui/path"

import { LineIcon, lniClass } from "../../../../z_details/_ui/icon/line_icon"

import { MenuPermission, MenuTree, MenuTreeNode } from "../infra"

export function category(
    label: string,
    permission: MenuPermission,
    children: MenuTree,
): MenuTreeNode {
    return { type: "category", category: { label, permission }, children }
}

export function item(label: string, icon: LineIcon, path: StaticMenuPath): MenuTreeNode {
    return { type: "item", item: { label, icon: lniClass(icon), path } }
}

export function assertMenuPath(path: string): StaticMenuPath {
    if (staticMenuPath.filter((staticPath) => staticPath === path).length === 0) {
        throw new Error(`path is not included in static-path: ${path}`)
    }
    return path as StaticMenuPath
}
