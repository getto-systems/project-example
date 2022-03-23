import { Clock } from "../../../z_lib/ui/clock/infra"

import { AuthTicket } from "../kernel/data"

export function convertCheckRemote(clock: Clock, roles: readonly string[]): AuthTicket {
    // remote からの値はバリデーションせずに受け取る
    return markAuthTicket(clock.now(), roles)
}

function markAuthTicket(authAt: Date, roles: readonly string[]): AuthTicket {
    return { authAt, grantedRoles: roles } as AuthTicket
}
