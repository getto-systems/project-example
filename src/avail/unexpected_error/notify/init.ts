import { newNotifyUnexpectedErrorRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"

import { NotifyUnexpectedErrorInfra } from "./infra"

export function newNotifyUnexpectedErrorInfra(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorInfra {
    return {
        notify: newNotifyUnexpectedErrorRemote(feature),
    }
}
