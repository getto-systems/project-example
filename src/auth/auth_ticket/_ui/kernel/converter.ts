import { RepositoryConverter } from "../../../../z_details/_ui/repository/infra"

import { AuthzRepositoryValue } from "./infra"
import { Clock } from "../../../../z_details/_ui/clock/infra"
import { AuthRemoteValue, AuthnRepositoryValue } from "./infra"

import { Authn, Authz, GrantedRoles } from "./data"
import { AuthAt, AuthTicket } from "./data"

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

export function convertAuthRemote(clock: Clock, value: AuthRemoteValue): AuthTicket {
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
