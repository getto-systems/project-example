import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { SearchPageResponse } from "../../../../z_lib/ui/remote/search/data"
import { UserAccount } from "../kernel/data"

export type SearchUserAccountFields = Readonly<{
    offset: BoardValue
    loginID: BoardValue
}>

export type SearchUserAccountError = SearchUserAccountRemoteError
export type SearchUserAccountRemoteError = RemoteCommonError

export type SearchUserAccountRemoteResponse = Readonly<{
    page: SearchPageResponse
    users: UserAccount[]
}>
