import { waitSecond } from "../../z_lib/ui/config/infra"

export const COMMON_CONFIG = {
    takeLongtimeThreshold: waitSecond(1),
    resetToInitialTimeout: waitSecond(1),
} as const
