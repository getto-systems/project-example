import { newNotifyUnexpectedErrorInfra } from "../notify/init"

import { initNotifyUnexpectedErrorResource } from "./impl"
import { initNotifyUnexpectedErrorCoreAction } from "./core/impl"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"

import { NotifyUnexpectedErrorResource } from "./resource"

type OutsideFeature = RemoteOutsideFeature
export function newNotifyUnexpectedErrorResource(
    feature: OutsideFeature,
): NotifyUnexpectedErrorResource {
    return initNotifyUnexpectedErrorResource(
        initNotifyUnexpectedErrorCoreAction(newNotifyUnexpectedErrorInfra(feature)),
    )
}
