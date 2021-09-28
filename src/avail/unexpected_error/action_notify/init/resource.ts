import { newNotifyUnexpectedErrorInfra } from "../../notify/init"

import { initNotifyUnexpectedErrorAction } from "../init"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"

import { NotifyUnexpectedErrorResource } from "../resource"

type OutsideFeature = RemoteOutsideFeature
export function newNotifyUnexpectedErrorResource(
    feature: OutsideFeature,
): NotifyUnexpectedErrorResource {
    return {
        error: initNotifyUnexpectedErrorAction(newNotifyUnexpectedErrorInfra(feature)),
    }
}
