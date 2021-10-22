import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import {
    SearchUserAccountFields,
    SearchUserAccountRemoteError,
    SearchUserAccountRemoteResponse,
} from "./data"

export interface SearchUserAccountFieldsDetecter {
    (): SearchUserAccountFields
}
export interface UpdateSearchUserAccountFieldsQuery {
    (fields: SearchUserAccountFields): void
}

export type SearchUserAccountInfra = Readonly<{
    search: SearchUserAccountRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface SearchUserAccountRemote {
    (fields: SearchUserAccountFields): Promise<SearchUserAccountRemoteResult>
}
export type SearchUserAccountRemoteResult = RemoteResult<
    SearchUserAccountRemoteResponse,
    SearchUserAccountRemoteError
>
