import { StaticMenuPath } from "../../../../y_environment/path"

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
