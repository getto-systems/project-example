import { Season } from "../core/season/kernel/data"

export const beginningOfSeason: readonly Season[] = [
    { year: 2021, period: "summer" } as Season, // winter 始まりだったら summer を除く
    { year: 2021, period: "winter" } as Season, // 動的にやろうとするとめんどくさいのでこうする
]
