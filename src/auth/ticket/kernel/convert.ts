import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"

import { Clock } from "../../../z_lib/ui/clock/infra"
import { AuthRemoteValue, AuthTicketRepositoryValue } from "./infra"

import { AuthTicket } from "./data"

export const authTicketRepositoryConverter: RepositoryConverter<
    AuthTicket,
    AuthTicketRepositoryValue
> = {
    toRepository: (value) => ({
        authAt: value.authAt.toISOString(),
        roles: value.roles,
    }),
    fromRepository: (value) => {
        const authAt = new Date(value.authAt)

        if (!authAt) {
            return { valid: false }
        }

        return {
            valid: true,
            value: markAuthProfile(authAt, value.roles),
        }
    },
}

export function convertAuthRemote(clock: Clock, value: AuthRemoteValue): AuthTicket {
    // remote からの値はバリデーションせずに受け取る
    return markAuthProfile(clock.now(), value.roles)
}

function markAuthProfile(authAt: Date, roles: string[]): AuthTicket {
    return { authAt, roles } as AuthTicket
}
