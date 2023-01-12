import { FetchRepositoryResult, StoreRepositoryResult } from "../repository/infra"

import { SidebarExpand } from "./data"

export interface ToggleSidebarRepository {
    get(): Promise<FetchRepositoryResult<SidebarExpand>>
    set(value: SidebarExpand): Promise<StoreRepositoryResult>
}
