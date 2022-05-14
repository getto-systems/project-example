import { waitSecond } from "../../z_lib/ui/config/infra"

export const AVAIL_CONFIG = {
    takeLongtimeThreshold: waitSecond(1),
} as const
