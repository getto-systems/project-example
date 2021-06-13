import { newNotifyUnexpectedErrorInfra } from "../notify_unexpected_error/init"

import { initNotifyUnexpectedErrorResource } from "./impl"
import { initNotifyUnexpectedErrorCoreAction } from "./core/impl"

import { RemoteOutsideFeature } from "../../../../ui/vendor/getto-application/infra/remote/feature"

import { NotifyUnexpectedErrorResource } from "./resource"

type OutsideFeature = RemoteOutsideFeature
export function newNotifyUnexpectedErrorResource(
    feature: OutsideFeature,
): NotifyUnexpectedErrorResource {
    return initNotifyUnexpectedErrorResource(
        initNotifyUnexpectedErrorCoreAction(newNotifyUnexpectedErrorInfra(feature)),
    )
}
