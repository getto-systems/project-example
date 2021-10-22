import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { RemoteCommonError, SearchPage } from "../../../../z_lib/ui/remote/data"
import { UserAccount } from "../kernel/data"

export type SearchUserAccountFields = Readonly<{
    loginID: BoardValue
}>

export type SearchUserAccountError = SearchUserAccountRemoteError
export type SearchUserAccountRemoteError = RemoteCommonError

export type SearchUserAccountRemoteResponse = Readonly<{ page: SearchPage; users: UserAccount[] }>
