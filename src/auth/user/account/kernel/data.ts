import { LoginId } from "../../login_id/kernel/data"
import { ResetTokenDestination } from "../../password/reset/token_destination/kernel/data"
import { AuthRole } from "../../kernel/data"

export type AuthUserAccount = Readonly<{
    loginId: LoginId
    grantedRoles: readonly AuthRole[]
    resetTokenDestination: ResetTokenDestination
    memo: AuthUserMemo
}>

export type AuthUserMemo = string & { AuthUserMemo: never }
