import { RemoteResult } from "../../../../common/util/remote/infra"

import { RemoteCommonError } from "../../../../common/util/remote/data"
import { SearchAuthUserAccountFilterData, SearchAuthUserAccountRemoteResponse } from "./data"
import { AuthUserAccount } from "../kernel/data"
import { DetectFocusListKeyResult } from "../../../../common/util/list/data"

export interface SearchAuthUserAccountFilterDetecter {
    (): SearchAuthUserAccountFilterData
}
export interface UpdateSearchAuthUserAccountFieldsQuery {
    (fields: SearchAuthUserAccountFilterData): void
}

export interface FocusAuthUserAccountDetecter {
    (): DetectFocusListKeyResult
}
export interface UpdateFocusAuthUserAccountQuery {
    focus(user: AuthUserAccount): void
    clear(): void
}

export interface SearchAuthUserAccountRemote {
    (fields: SearchAuthUserAccountFilterData): Promise<SearchAuthUserAccountRemoteResult>
}
export type SearchAuthUserAccountRemoteResult = RemoteResult<
    SearchAuthUserAccountRemoteResponse,
    RemoteCommonError
>
