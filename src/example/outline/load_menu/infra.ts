import { RemoteResult } from "../../../z_lib/ui/remote/infra"
import { MenuBadge, MenuExpand } from "../kernel/infra"
import { FetchRepositoryResult, StoreRepositoryResult } from "../../../z_lib/ui/repository/infra"

import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export interface GetMenuBadgeRemote {
    (): Promise<GetMenuBadgeRemoteResult>
}
export type GetMenuBadgeRemoteResult = RemoteResult<MenuBadge, RemoteCommonError>

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
export type MenuExpandRepositoryValue = string[][]
