import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import {  OverrideLoginIdConfig } from "../action"

export function newOverridePasswordConfig(): OverrideLoginIdConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
