import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"
import { initMenuExpand, MenuBadge, MenuBadgeItem, MenuExpand } from "../kernel/infra"
import { MenuExpandRepositoryValue } from "./infra"

import { MenuCategoryLabel } from "../kernel/data"

export function convertMenuBadgeRemote(menuBadgeItems: MenuBadgeItem[]): MenuBadge {
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

function markMenuCategoryLabel(label: string): MenuCategoryLabel {
    return label as MenuCategoryLabel
}
