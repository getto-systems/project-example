import { GrantedAuthRole } from "../../user/account/input/data"

export type AuthTicket = Readonly<{
    authAt: Date
    grantedRoles: readonly GrantedAuthRole[]
}> & { AuthTicket: never }
