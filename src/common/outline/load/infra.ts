import { RemoteResult } from "../../util/remote/infra"
import { FetchRepositoryResult, StoreRepositoryResult } from "../../util/repository/infra"

import { RemoteCommonError } from "../../util/remote/data"
import { ConvertLocationResult } from "../../util/location/data"
import { MenuCategoryPath, MenuTargetPath } from "./data"
import { Icon } from "../../util/icon/data"
import { AuthPermission } from "../../../auth/user/kernel/data"

export interface MenuTargetPathDetecter {
    (): ConvertLocationResult<MenuTargetPath>
}

export type MenuContent = Readonly<{
    key: string
    loadMenuBadge: boolean
    menuTree: MenuTree
}>

export type MenuTree = readonly MenuTreeNode[]
export type MenuTreeNode =
    | Readonly<{ type: "category"; category: MenuTreeCategory; children: MenuTree }>
    | Readonly<{ type: "item"; item: MenuTreeItem }>

export type MenuTreeCategory = Readonly<{
    label: MenuTreeLabel
    required: MenuPermissionRequired
}>
export type MenuTreeItem = Readonly<{
    path: MenuPath
    label: string
    icon: Icon
}>

export type MenuTreeLabel = string
export type MenuPath = string

export type MenuPermissionRequired =
    | Readonly<{ type: "nothing" }>
    | Readonly<{ type: "has-some"; permissions: readonly AuthPermission[] }>

export type MenuBadge = Map<string, number>
export type MenuBadgeItem = Readonly<{ path: string; count: number }>

export type MenuExpand = ArraySet<MenuCategoryPath>

class ArraySet<T> {
    values: T[] = []
    equals: ArraySetEntryEquals<T>

    constructor(equals: ArraySetEntryEquals<T>) {
        this.equals = equals
    }

    init(set: T[]): void {
        set.forEach((entry) => {
            this.register(entry)
        })
    }
    register(entry: T): void {
        if (this.hasEntry(entry)) {
            return
        }
        this.values = [...this.values, entry]
    }
    remove(entry: T): void {
        this.values = this.values.filter((value) => !this.equals(entry, value))
    }
    hasEntry(entry: T): boolean {
        return this.values.some((value) => this.equals(entry, value))
    }
}
interface ArraySetEntryEquals<T> {
    (a: T, b: T): boolean
}

export function initMenuExpand(): MenuExpand {
    return new ArraySet<MenuCategoryPath>((a, b) => {
        if (a.length !== b.length) {
            return false
        }
        for (let i = 0; i < a.length; i++) {
            if (a[i] !== b[i]) {
                return false
            }
        }
        return true
    })
}

export interface LoadMenuBadgeRemote {
    (): Promise<LoadMenuBadgeRemoteResult>
}
export type LoadMenuBadgeRemoteResult = RemoteResult<MenuBadge, RemoteCommonError>

export type MenuBadgeStore = MenuStore<MenuBadge>
export type MenuExpandStore = MenuStore<MenuExpand>

export interface MenuStore<T> {
    get(): FetchMenuStoreResult<T>
    set(value: T): void
}
export type FetchMenuStoreResult<T> =
    | Readonly<{ found: true; value: T }>
    | Readonly<{ found: false }>

export interface MenuExpandRepository {
    get(): Promise<FetchRepositoryResult<MenuExpand>>
    set(value: MenuExpand): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type MenuExpandRepositoryValue = readonly ReadonlyArray<string>[]
