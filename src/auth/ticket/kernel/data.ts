import { AuthRole } from "../../user/kernel/data"

export type AuthTicket = Readonly<{
    authAt: Date
    grantedRoles: readonly AuthRole[]
}> & { AuthTicket: never }
