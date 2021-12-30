import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"

import { Clock } from "../../../z_lib/ui/clock/infra"
import { AuthRemoteValue, AuthProfileRepositoryValue } from "./infra"

import { AuthProfile } from "./data"

export const authProfileRepositoryConverter: RepositoryConverter<
    AuthProfile,
    AuthProfileRepositoryValue
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

export function convertAuthRemote(clock: Clock, value: AuthRemoteValue): AuthProfile {
    // remote からの値はバリデーションせずに受け取る
    return markAuthProfile(clock.now(), value.roles)
}

function markAuthProfile(authAt: Date, roles: string[]): AuthProfile {
    return { authAt, roles } as AuthProfile
}
