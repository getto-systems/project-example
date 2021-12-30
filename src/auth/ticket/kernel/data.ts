import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export type AuthProfile = Readonly<{
    authAt: Date
    roles: string[]
}> & { AuthProfile: never }

export function hasExpired(
    { authAt }: AuthProfile,
    target: { now: Date; expire_millisecond: number },
): boolean {
    return target.now.getTime() > authAt.getTime() + target.expire_millisecond
}

export type RenewAuthTicketError = RenewAuthTicketRemoteError
export type RenewAuthTicketRemoteError = RemoteCommonError
