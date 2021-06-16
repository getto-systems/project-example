import { newNotifyUnexpectedErrorRemote } from "./infra/remote/notify"

import { RemoteOutsideFeature } from "../../../z_details/_ui/remote/feature"

import { NotifyUnexpectedErrorInfra } from "./infra"

export function newNotifyUnexpectedErrorInfra(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorInfra {
    return {
        notify: newNotifyUnexpectedErrorRemote(feature),
    }
}
