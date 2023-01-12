import { Icon } from "../../util/icon/data"

export type BreadcrumbList = readonly BreadcrumbNode[]

export type BreadcrumbNode =
    | Readonly<{ type: "category"; category: MenuCategory }>
    | Readonly<{ type: "item"; item: MenuItem }>

export type MenuTargetPath = string & { MenuTarget: never }

export type Menu = readonly MenuNode[]
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

export type MenuCategoryPath = readonly MenuCategoryLabel[]

export type MenuItem = MenuItem_data & { MenuItem: never }
type MenuItem_data = Readonly<{
    label: string
    icon: Icon
    href: string
}>
