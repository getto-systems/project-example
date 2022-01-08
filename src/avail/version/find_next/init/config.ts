import { AVAIL_CONFIG } from "../../../x_outside_feature/config"
import { FindNextVersionConfig } from "../action"

export function newFindNextVersionConfig(): FindNextVersionConfig {
    return {
        takeLongtimeThreshold: AVAIL_CONFIG.takeLongtimeThreshold,
    }
}
