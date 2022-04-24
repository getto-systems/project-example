import { toGrantedRoles } from "../../user/account/input/granted_roles/convert"

import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"
import { AuthTicketRepositoryValue } from "./infra"

import { AuthTicket } from "./data"

export const authTicketRepositoryConverter: RepositoryConverter<
    AuthTicket,
    AuthTicketRepositoryValue
> = {
    toRepository: (value) => ({
        authAt: value.authAt.toISOString(),
        grantedRoles: value.grantedRoles,
    }),
    fromRepository: (value) => {
        const authAt = new Date(value.authAt)

        if (!authAt) {
            return { valid: false }
        }

        return {
            valid: true,
            value: markAuthTicket(authAt, value.grantedRoles),
        }
    },
}

function markAuthTicket(authAt: Date, roles: readonly string[]): AuthTicket {
    return { authAt, grantedRoles: toGrantedRoles(roles) } as AuthTicket
}
