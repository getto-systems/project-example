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

export const AUTH_USER_ACCOUNT = {
    "login-id": "ログインID",
    password: "パスワード",
    memo: "備考",
    "granted-roles": "権限",
    "reset-token-destination": "パスワードリセット用Eメール",
} as const
