export type ExpireTime = Readonly<{ expire_millisecond: number }>
export type IntervalTime = Readonly<{ interval_millisecond: number }>
export type WaitTime = Readonly<{ wait_millisecond: number }>
export type Limit = Readonly<{ limit: number }>

export function expireSecond(second: number): ExpireTime {
    return { expire_millisecond: second * 1000 }
}
export function expireMinute(minute: number): ExpireTime {
    return expireSecond(minute * 60)
}
export function expireHour(hour: number): ExpireTime {
    return expireMinute(hour * 60)
}
export function expireDay(day: number): ExpireTime {
    return expireHour(day * 24)
}

export function intervalSecond(second: number): IntervalTime {
    return { interval_millisecond: second * 1000 }
}
export function intervalMinute(minute: number): IntervalTime {
    return intervalSecond(minute * 60)
}

export function waitSecond(second: number): WaitTime {
    return { wait_millisecond: second * 1000 }
}
export function waitMinute(minute: number): WaitTime {
    return waitSecond(minute * 60)
}

export function limit(count: number): Limit {
    return { limit: count }
}
