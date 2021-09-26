import { RemoteCommonError } from "../../../z_details/_ui/remote/data"

export type AuthTicket = Readonly<{
    authn: Authn
    authz: Authz
}>

export type Authn = Readonly<{
    authAt: AuthAt
}>

export type Authz = Readonly<{
    roles: GrantedRoles
}>

export type AuthAt = Date & { AuthAt: never }
export type GrantedRoles = string[] & { GrantedRoles: never }

export function hasExpired(
    authAt: AuthAt,
    target: { now: Date; expire_millisecond: number },
): boolean {
    return target.now.getTime() > authAt.getTime() + target.expire_millisecond
}

export type RenewAuthTicketError = RenewAuthTicketRemoteError
export type RenewAuthTicketRemoteError = RemoteCommonError
