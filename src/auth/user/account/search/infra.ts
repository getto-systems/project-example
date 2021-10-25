import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import {
    SearchAuthUserAccountFields,
    SearchAuthUserAccountRemoteError,
    SearchAuthUserAccountRemoteResponse,
} from "./data"

export type SearchAuthUserAccountFieldsDetectParams = Readonly<{
    defaultSortKey: string
}>
export interface SearchAuthUserAccountFieldsDetecter {
    (params: SearchAuthUserAccountFieldsDetectParams): SearchAuthUserAccountFields
}
export interface UpdateSearchAuthUserAccountFieldsQuery {
    (fields: SearchAuthUserAccountFields): void
}

export type SearchAuthUserAccountInfra = Readonly<{
    search: SearchAuthUserAccountRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface SearchAuthUserAccountRemote {
    (fields: SearchAuthUserAccountFields): Promise<SearchAuthUserAccountRemoteResult>
}
export type SearchAuthUserAccountRemoteResult = RemoteResult<
    SearchAuthUserAccountRemoteResponse,
    SearchAuthUserAccountRemoteError
>
