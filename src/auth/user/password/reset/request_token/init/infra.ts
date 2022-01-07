import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { newRequestResetTokenRemote } from "./request_token_remote"

import { RequestResetTokenInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenInfra(feature: OutsideFeature): RequestResetTokenInfra {
    return {
        requestTokenRemote: newRequestResetTokenRemote(feature),
    }
}
