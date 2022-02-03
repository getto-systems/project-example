import { EXAMPLE_CONFIG } from "../../../x_outside_feature/config"

import { FocusSeasonConfig } from "../action"

export function newFocusSeasonConfig(): FocusSeasonConfig {
    return {
        focusSeasonExpire: EXAMPLE_CONFIG.focusSeasonExpire,
    }
}
