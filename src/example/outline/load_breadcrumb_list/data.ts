import { MenuCategory, MenuItem } from "../kernel/data"

export type BreadcrumbList = readonly BreadcrumbNode[]

export type BreadcrumbNode =
    | Readonly<{ type: "category"; category: MenuCategory }>
    | Readonly<{ type: "item"; item: MenuItem }>
