import { toMenuCategory, toMenuItem } from "../kernel/impl/converter"

import { MenuTree, MenuTreeCategory, MenuTreeItem, MenuTreeNode } from "../kernel/infra"
import { LoadBreadcrumbListInfra } from "./infra"

import { LoadBreadcrumbListPod } from "./method"

import { MenuTargetPath } from "../kernel/data"
import { BreadcrumbList, BreadcrumbNode } from "./data"

interface Load {
    (infra: LoadBreadcrumbListInfra): LoadBreadcrumbListPod
}
export const loadBreadcrumbList: Load = (infra) => (detecter) => () => {
    const { version } = infra
    const target = detecter()
    if (!target.valid) {
        return EMPTY
    }
    return build(target.value)

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
