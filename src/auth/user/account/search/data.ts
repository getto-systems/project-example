import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { SearchPageResponse } from "../../../../z_lib/ui/search/data"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { AuthUserAccountBasket } from "../kernel/data"

export type SearchAuthUserAccountFields = Readonly<{
    offset: BoardValue
    sort: SearchSort
    loginID: BoardValue
}>

export type SearchAuthUserAccountRemoteResponse = Readonly<{
    page: SearchPageResponse
    users: readonly AuthUserAccountBasket[]
}>
