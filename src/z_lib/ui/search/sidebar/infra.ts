import { FetchRepositoryResult, StoreRepositoryResult } from "../../repository/infra"

import { SearchSidebarExpand } from "./data"

export interface SearchSidebarRepository {
    get(): Promise<FetchRepositoryResult<SearchSidebarExpand>>
    set(value: SearchSidebarExpand): Promise<StoreRepositoryResult>
}
