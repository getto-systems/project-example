import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { SearchAuthUserAccountFields, SearchAuthUserAccountRemoteResponse } from "./data"

export type SearchAuthUserAccountFieldsDetectParams = Readonly<{
    defaultSortKey: string
}>
export interface SearchAuthUserAccountFieldsDetecter {
    (params: SearchAuthUserAccountFieldsDetectParams): SearchAuthUserAccountFields
}
export interface UpdateSearchAuthUserAccountFieldsQuery {
    (fields: SearchAuthUserAccountFields): void
}

export interface SearchAuthUserAccountRemote {
    (fields: SearchAuthUserAccountFields): Promise<SearchAuthUserAccountRemoteResult>
}
export type SearchAuthUserAccountRemoteResult = RemoteResult<
    SearchAuthUserAccountRemoteResponse,
    RemoteCommonError
>
