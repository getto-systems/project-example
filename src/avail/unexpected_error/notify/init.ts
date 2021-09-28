import { newNotifyUnexpectedErrorRemote } from "./init/remote/notify"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"

import { NotifyUnexpectedErrorInfra } from "./infra"

export function newNotifyUnexpectedErrorInfra(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorInfra {
    return {
        notify: newNotifyUnexpectedErrorRemote(feature),
    }
}
