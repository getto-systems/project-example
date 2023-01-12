import { Clock } from "../../../common/util/clock/infra"

import { AuthTicket } from "../kernel/data"

export function convertCheckRemote(clock: Clock, permissions: readonly string[]): AuthTicket {
    // remote からの値はバリデーションせずに受け取る
    return markAuthTicket(clock.now(), permissions)
}

function markAuthTicket(authAt: Date, permissions: readonly string[]): AuthTicket {
    return { authAt, granted: permissions } as AuthTicket
}
