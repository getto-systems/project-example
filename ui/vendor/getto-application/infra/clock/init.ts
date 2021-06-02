import { Clock } from "./infra"

export function newClock(): Clock {
    return {
        now: () => new Date(),
    }
}
