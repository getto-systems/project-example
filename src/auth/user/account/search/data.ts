import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { SearchPageResponse } from "../../../../z_lib/ui/search/data"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { AuthUserAccountBasket } from "../kernel/data"

export type SearchAuthUserAccountFields = Readonly<{
    offset: BoardValue
    sort: SearchSort
    loginID: BoardValue
}>

export type SearchAuthUserAccountError = SearchAuthUserAccountRemoteError
export type SearchAuthUserAccountRemoteError = RemoteCommonError

export type SearchAuthUserAccountRemoteResponse = Readonly<{
    page: SearchPageResponse
    summary: {
        /* no props */
    }
    users: AuthUserAccountBasket[]
}>