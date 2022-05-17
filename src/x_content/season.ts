import { Season } from "../common/season/kernel/data"

export const beginningOfSystemSeason: readonly Season[] = [
    { year: 2021, period: "summer" } as Season, // winter 始まりだったら summer を除く
    { year: 2021, period: "winter" } as Season, // 動的にやろうとするとめんどくさいのでこうする
]
