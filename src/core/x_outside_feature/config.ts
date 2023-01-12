import { expireDay } from "../../common/util/config/infra"

export const CORE_CONFIG = {
    manualSetupSeasonExpire: expireDay(90),
} as const
