import { Clock } from "../../../../z_lib/ui/clock/infra"
import { Season } from "../data"

export function defaultSeason(clock: Clock): Season {
    const now = clock.now()
    const year = now.getFullYear()
    const month = now.getMonth()

    if (month < 3) {
        // 1, 2, 3月は前の年の winter
        return { year: year - 1, period: "winter" } as Season
    }
    if (month > 8) {
        // 10, 11, 12月はその年の winter
        return { year, period: "winter" } as Season
    }
    // 4, 5, 6, 7, 8, 9月はその年の summer
    return { year, period: "summer" } as Season
}
