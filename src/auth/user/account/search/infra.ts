import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { SearchAuthUserAccountFilter, SearchAuthUserAccountRemoteResponse } from "./data"

export interface SearchAuthUserAccountFilterDetecter {
    (): SearchAuthUserAccountFilter
}
export interface UpdateSearchAuthUserAccountFieldsQuery {
    (fields: SearchAuthUserAccountFilter): void
}

export interface SearchAuthUserAccountRemote {
    (fields: SearchAuthUserAccountFilter): Promise<SearchAuthUserAccountRemoteResult>
}
export type SearchAuthUserAccountRemoteResult = RemoteResult<
    SearchAuthUserAccountRemoteResponse,
    RemoteCommonError
>
