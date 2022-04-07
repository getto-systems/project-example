import { LoginId } from "../../login_id/input/data"
import { ResetTokenDestination } from "../../password/reset/token_destination/kernel/data"
import { GrantedAuthRole } from "../input/data"

export type AuthUserAccount = Readonly<{
    loginId: LoginId
    grantedRoles: readonly GrantedAuthRole[]
    resetTokenDestination: ResetTokenDestination
}>
