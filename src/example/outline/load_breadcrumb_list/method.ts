import { MenuTree, MenuTreeCategory, MenuTreeItem, MenuTreeNode } from "../kernel/infra"
import { LoadBreadcrumbListInfra } from "./infra"

import { toMenuCategory, toMenuItem } from "../kernel/convert"

import { BreadcrumbList, BreadcrumbNode } from "./data"
import { MenuTargetPath } from "../kernel/data"
import { ConvertLocationResult } from "../../../z_details/_ui/location/data"

export interface LoadBreadcrumbListMethod {
    (menuTargetPath: ConvertLocationResult<MenuTargetPath>): BreadcrumbList
}

interface Load {
    (infra: LoadBreadcrumbListInfra): LoadBreadcrumbListMethod
}
export const loadBreadcrumbList: Load = (infra) => (menuTargetPath) => {
    const { version } = infra
    if (!menuTargetPath.valid) {
        return EMPTY
    }
    return build(menuTargetPath.value)

    function build(currentPath: MenuTargetPath): BreadcrumbList {
        return toBreadcrumb(infra.menuTree)

        function toBreadcrumb(tree: MenuTree): BreadcrumbList {
            for (let i = 0; i < tree.length; i++) {
                const breadcrumbList = findFocusedNode(tree[i])
                if (breadcrumbList.length > 0) {
                    return breadcrumbList
                }
            }
            return EMPTY
        }
        function findFocusedNode(node: MenuTreeNode): BreadcrumbNode[] {
            switch (node.type) {
                case "category":
                    return categoryNode(node.category, node.children)
                case "item":
                    return itemNode(node.item)
            }
        }
        function categoryNode(category: MenuTreeCategory, children: MenuTree): BreadcrumbNode[] {
            const breadcrumb = toBreadcrumb(children)
            if (breadcrumb.length === 0) {
                return EMPTY
            }
            return [{ type: "category", category: toMenuCategory(category) }, ...breadcrumb]
        }
        function itemNode(item: MenuTreeItem): BreadcrumbNode[] {
            if (item.path !== currentPath) {
                return EMPTY
            }
            return [{ type: "item", item: toMenuItem(item, version) }]
        }
    }
}

const EMPTY: BreadcrumbList = []
