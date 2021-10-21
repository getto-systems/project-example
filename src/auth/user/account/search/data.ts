import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

export type SearchUserAccountFields = Readonly<{
    loginID: BoardValue
}>

export type SearchUserAccountError = SearchUserAccountRemoteError
export type SearchUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "invalid-search" }>
