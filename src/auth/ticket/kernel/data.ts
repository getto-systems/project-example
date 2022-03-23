import { GrantedRole } from "../../user/account/input/data"

export type AuthTicket = Readonly<{
    authAt: Date
    grantedRoles: readonly GrantedRole[]
}> & { AuthTicket: never }
