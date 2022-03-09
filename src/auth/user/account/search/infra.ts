import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { SearchAuthUserAccountFilter, SearchAuthUserAccountRemoteResponse } from "./data"
import { AuthUserAccountBasket } from "../kernel/data"

export interface SearchAuthUserAccountFilterDetecter {
    (): SearchAuthUserAccountFilter
}
export interface UpdateSearchAuthUserAccountFieldsQuery {
    (fields: SearchAuthUserAccountFilter): void
}

export interface FocusAuthUserAccountDetecter {
    (): DetectLoginIdResult
}
export interface UpdateFocusAuthUserAccountQuery {
    focus(user: AuthUserAccountBasket): void
    clear(): void
}

export type DetectLoginIdResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; loginId: string }>

export interface SearchAuthUserAccountRemote {
    (fields: SearchAuthUserAccountFilter): Promise<SearchAuthUserAccountRemoteResult>
}
export type SearchAuthUserAccountRemoteResult = RemoteResult<
    SearchAuthUserAccountRemoteResponse,
    RemoteCommonError
>
