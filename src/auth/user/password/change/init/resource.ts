import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newChangePasswordConfig } from "./config"

import { ChangePasswordAction, initChangePasswordAction } from "../action"

import { newChangePasswordInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordResource(
    feature: OutsideFeature,
): Readonly<{ change: ChangePasswordAction }> {
    return {
        change: initChangePasswordAction({
            infra: newChangePasswordInfra(feature),
            config: newChangePasswordConfig(),
        }),
    }
}
