import { GrantedRoles } from "../../../ticket/kernel/data"
import { LoginID } from "../../login_id/input/data"

export type UserAccount = Readonly<{
    loginID: LoginID
    grantedRoles: GrantedRoles
}>
