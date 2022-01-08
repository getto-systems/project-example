import { BreadcrumbList, BreadcrumbNode } from "../load_breadcrumb_list/data"
import {
    MenuTargetPathDetecter,
    MenuTree,
    MenuTreeCategory,
    MenuTreeItem,
    MenuTreeNode,
} from "../kernel/infra"
import { MenuTargetPath } from "../kernel/data"
import { toMenuCategory, toMenuItem } from "../kernel/convert"

export interface LoadBreadcrumbListAction {
    load(): BreadcrumbList
}

export type LoadBreadcrumbListInfra = Readonly<{
    version: string
    menuTree: MenuTree
}>
export type LoadBreadcrumbListShell = Readonly<{
    detectTargetPath: MenuTargetPathDetecter
}>

export function initLoadBreadcrumbListAction(
    infra: LoadBreadcrumbListInfra,
    shell: LoadBreadcrumbListShell,
): LoadBreadcrumbListAction {
    return {
        load: () => load(infra, shell),
    }
}

function load(infra: LoadBreadcrumbListInfra, shell: LoadBreadcrumbListShell): BreadcrumbList {
    const { version } = infra
    const { detectTargetPath } = shell

    const menuTargetPath = detectTargetPath()
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
