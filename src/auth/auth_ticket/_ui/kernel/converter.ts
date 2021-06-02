import { RepositoryConverter } from "../../../../../ui/vendor/getto-application/infra/repository/infra"

import {  AuthzRepositoryValue } from "./infra"
import { Clock } from "../../../../../ui/vendor/getto-application/infra/clock/infra"
import { RemoteConverter } from "../../../../../ui/vendor/getto-application/infra/remote/infra"
import { AuthRemoteValue, AuthnRepositoryValue } from "./infra"

import { Authn, Authz,  GrantedRoles } from "./data"
import { AuthAt, AuthTicket,  } from "./data"

export const authnRepositoryConverter: RepositoryConverter<Authn, AuthnRepositoryValue> = {
    toRepository: (value) => ({
        authAt: value.authAt.toISOString(),
    }),
    fromRepository: (value) => {
        const authAt = new Date(value.authAt)

        if (!authAt) {
            return { valid: false }
        }

        return {
            valid: true,
            value: {
                authAt: markAuthAt(authAt),
            },
        }
    },
}

interface AuthConverter {
    (clock: Clock): RemoteConverter<AuthTicket, AuthRemoteValue>
}
export const authRemoteConverter: AuthConverter = (clock) => (value) => {
    // remote からの値はバリデーションせずに受け取る
    return {
        authn: {
            authAt: markAuthAt(clock.now()),
        },
        authz: authzRemoteConverter(value.roles),
    }
}

export const authzRepositoryConverter: RepositoryConverter<Authz, AuthzRepositoryValue> = {
    toRepository: (value) => value,
    fromRepository: (value) => {
        const roles = value.roles

        // roles のバリデーションは特にしない

        return {
            valid: true,
            found: true,
            value: {
                roles: markAuthzRoles(roles),
            },
        }
    },
}

export function authzRemoteConverter(roles: string[]): Authz {
    return {
        roles: markAuthzRoles(roles),
    }
}

function markAuthAt(date: Date): AuthAt {
    return date as AuthAt
}
function markAuthzRoles(roles: string[]): GrantedRoles {
    return roles as GrantedRoles
}
