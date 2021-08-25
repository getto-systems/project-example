import { auth_config } from "../../../../_ui/x_outside_feature/config"

import { newRequestResetTokenRemote } from "./init/remote/request_token"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { RequestResetTokenInfra } from "./infra"

export function newRequestResetTokenInfra(feature: RemoteOutsideFeature): RequestResetTokenInfra {
    return {
        requestToken: newRequestResetTokenRemote(feature),
        config: {
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}
