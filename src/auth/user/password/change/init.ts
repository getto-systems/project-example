import { AUTH_CONFIG } from "../../../x_outside_feature/config"

import { newChangePasswordRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { ChangePasswordInfra } from "./infra"

export function newChangePasswordInfra(feature: RemoteOutsideFeature): ChangePasswordInfra {
    return {
        change: newChangePasswordRemote(feature),
        config: {
            takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
        },
    }
}
