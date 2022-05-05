import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import {  OverwriteLoginIdConfig } from "../action"

export function newOverwritePasswordConfig(): OverwriteLoginIdConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
