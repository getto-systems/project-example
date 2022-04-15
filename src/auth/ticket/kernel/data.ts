import { GrantedAuthRole } from "../../user/kernel/data"

export type AuthTicket = Readonly<{
    authAt: Date
    grantedRoles: readonly GrantedAuthRole[]
}> & { AuthTicket: never }
