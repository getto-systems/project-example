import { LoginId } from "../../login_id/kernel/data"
import { ResetTokenDestination } from "../../password/reset/token_destination/kernel/data"
import { AuthRole } from "../../kernel/data"

export type AuthUserAccount = Readonly<{
    loginId: LoginId
    grantedRoles: readonly AuthRole[]
    resetTokenDestination: ResetTokenDestination
    memo: TypeAuthUser<"memo">
}>

type TypeDef<K extends string, T> = T & { [key in K]: never }
export type TypeAuthUser<K extends AuthUserField> = TypeDef<`authUser-${K}`, AuthUserExtract[K]>
export type AuthUserField = keyof AuthUserExtract
export type AuthUserExtract = {
    memo: string
}

export const AUTH_USER_ACCOUNT = {
    loginId: "ログインID",
    password: "パスワード",
    memo: "備考",
    grantedRoles: "権限",
    resetTokenDestination: "パスワードリセット用Eメール",
} as const
