import { RemoteResult } from "../../../../common/util/remote/infra"

import { RemoteCommonError } from "../../../../common/util/remote/data"
import { SearchAuthUserAccountFilter, SearchAuthUserAccountRemoteResponse } from "./data"
import { AuthUserAccount } from "../kernel/data"
import { DetectFocusListKeyResult } from "../../../../common/util/list/data"

export interface SearchAuthUserAccountFilterDetecter {
    (): SearchAuthUserAccountFilter
}
export interface UpdateSearchAuthUserAccountFieldsQuery {
    (fields: SearchAuthUserAccountFilter): void
}

export interface FocusAuthUserAccountDetecter {
    (): DetectFocusListKeyResult
}
export interface UpdateFocusAuthUserAccountQuery {
    focus(user: AuthUserAccount): void
    clear(): void
}

export interface SearchAuthUserAccountRemote {
    (fields: SearchAuthUserAccountFilter): Promise<SearchAuthUserAccountRemoteResult>
}
export type SearchAuthUserAccountRemoteResult = RemoteResult<
    SearchAuthUserAccountRemoteResponse,
    RemoteCommonError
>
