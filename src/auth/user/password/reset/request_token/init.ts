import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { newRequestResetTokenRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { RequestResetTokenInfra } from "./infra"

export function newRequestResetTokenInfra(feature: RemoteOutsideFeature): RequestResetTokenInfra {
    return {
        requestToken: newRequestResetTokenRemote(feature),
        config: {
            takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
        },
    }
}
