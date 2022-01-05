import { AuthTicket } from "./data"

export function hasExpired(
    { authAt }: AuthTicket,
    target: { now: Date; expire_millisecond: number },
): boolean {
    return target.now.getTime() > authAt.getTime() + target.expire_millisecond
}
