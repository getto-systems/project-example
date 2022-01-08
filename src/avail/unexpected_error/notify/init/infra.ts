import { newNotifyUnexpectedErrorRemote } from "./notify_remote"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { NotifyUnexpectedErrorInfra } from "../action"

export function newNotifyUnexpectedErrorInfra(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorInfra {
    return {
        notify: newNotifyUnexpectedErrorRemote(feature),
    }
}
