import { AUTH_CONFIG } from "../../../../../../x_outside_feature/config"

import { ChangeResetTokenDestinationConfig } from "../action"

export function newChangeResetTokenDestinationConfig(): ChangeResetTokenDestinationConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
