import { toGranted } from "../../user/account/input/granted/convert"

import { RepositoryConverter } from "../../../common/util/repository/infra"
import { AuthTicketRepositoryValue } from "./infra"

import { AuthTicket } from "./data"

export const authTicketRepositoryConverter: RepositoryConverter<
    AuthTicket,
    AuthTicketRepositoryValue
> = {
    toRepository: (value) => ({
        authAt: value.authAt.toISOString(),
        granted: value.granted,
    }),
    fromRepository: (value) => {
        const authAt = new Date(value.authAt)

        if (!authAt) {
            return { valid: false }
        }

        return {
            valid: true,
            value: markAuthTicket(authAt, value.granted),
        }
    },
}

function markAuthTicket(authAt: Date, permissions: readonly string[]): AuthTicket {
    return { authAt, granted: toGranted(permissions) } as AuthTicket
}
