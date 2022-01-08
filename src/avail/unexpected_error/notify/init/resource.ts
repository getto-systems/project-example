import { newNotifyUnexpectedErrorInfra } from "./infra"

import { initNotifyUnexpectedErrorAction, NotifyUnexpectedErrorAction } from "../../notify/action"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

type OutsideFeature = RemoteOutsideFeature
export function newNotifyUnexpectedErrorResource(
    feature: OutsideFeature,
): Readonly<{ error: NotifyUnexpectedErrorAction }> {
    return {
        error: initNotifyUnexpectedErrorAction(newNotifyUnexpectedErrorInfra(feature)),
    }
}
