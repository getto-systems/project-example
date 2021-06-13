import { newNotifyUnexpectedErrorRemote } from "../infra/remote/notify"

import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/feature"

import { NotifyUnexpectedErrorInfra } from "../infra"

export function newNotifyUnexpectedErrorInfra(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorInfra {
    return {
        notify: newNotifyUnexpectedErrorRemote(feature),
    }
}
