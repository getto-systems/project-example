import { RemoteCommonError } from "../../../../z_details/_ui/remote/data"

export type MenuTargetPath = string & { MenuTarget: never }

export type Menu = MenuNode[]
export type MenuNode = MenuCategoryNode | MenuItemNode
export type MenuCategoryNode = Readonly<{
    type: "category"
    category: MenuCategory
    path: MenuCategoryPath
    children: Menu
    isExpand: boolean
    badgeCount: number
}>
export type MenuItemNode = Readonly<{
    type: "item"
    item: MenuItem
    isActive: boolean
    badgeCount: number
}>

export type MenuCategory = Readonly<{
    label: MenuCategoryLabel
}>
export type MenuCategoryLabel = string & { MenuCategoryLabel: never }

export type MenuCategoryPath = MenuCategoryLabel[]

export type MenuItem = MenuItem_data & { MenuItem: never }
type MenuItem_data = Readonly<{
    label: string
    icon: string
    href: string
}>

export type GetMenuBadgeError = GetMenuBadgeRemoteError
export type GetMenuBadgeRemoteError = RemoteCommonError
