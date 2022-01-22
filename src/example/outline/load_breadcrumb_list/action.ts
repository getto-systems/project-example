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

export type LoadBreadcrumbListMaterial = Readonly<{
    shell: LoadBreadcrumbListShell
    config: LoadBreadcrumbListConfig
}>
export type LoadBreadcrumbListShell = Readonly<{
    detectTargetPath: MenuTargetPathDetecter
}>
export type LoadBreadcrumbListConfig = Readonly<{
    version: string
    menuTree: MenuTree
}>

export function initLoadBreadcrumbListAction(
    material: LoadBreadcrumbListMaterial,
): LoadBreadcrumbListAction {
    return {
        load: () => load(material),
    }
}

function load({ shell, config }: LoadBreadcrumbListMaterial): BreadcrumbList {
    const { detectTargetPath } = shell

    const menuTargetPath = detectTargetPath()
    if (!menuTargetPath.valid) {
        return EMPTY
    }
    return build(menuTargetPath.value)

    function build(currentPath: MenuTargetPath): BreadcrumbList {
        return toBreadcrumb(config.menuTree)

        function toBreadcrumb(tree: MenuTree): BreadcrumbList {
            for (let i = 0; i < tree.length; i++) {
                const breadcrumbList = findFocusedNode(tree[i])
                if (breadcrumbList.length > 0) {
                    return breadcrumbList
                }
            }
            return EMPTY
        }
        function findFocusedNode(node: MenuTreeNode): readonly BreadcrumbNode[] {
            switch (node.type) {
                case "category":
                    return categoryNode(node.category, node.children)
                case "item":
                    return itemNode(node.item)
            }
        }
        function categoryNode(
            category: MenuTreeCategory,
            children: MenuTree,
        ): readonly BreadcrumbNode[] {
            const breadcrumb = toBreadcrumb(children)
            if (breadcrumb.length === 0) {
                return EMPTY
            }
            return [{ type: "category", category: toMenuCategory(category) }, ...breadcrumb]
        }
        function itemNode(item: MenuTreeItem): readonly BreadcrumbNode[] {
            if (item.path !== currentPath) {
                return EMPTY
            }
            return [{ type: "item", item: toMenuItem(item, config.version) }]
        }
    }
}

const EMPTY: BreadcrumbList = []
