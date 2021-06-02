import { newNotifyUnexpectedErrorInfra } from "../notify_unexpected_error/impl/init"

import { initNotifyUnexpectedErrorResource } from "./impl"
import { initNotifyUnexpectedErrorCoreAction } from "./core/impl"

import { NotifyUnexpectedErrorResource } from "./resource"
import { RemoteOutsideFeature } from "../../../../ui/vendor/getto-application/infra/remote/infra"

type OutsideFeature = RemoteOutsideFeature
export function newNotifyUnexpectedErrorResource(
    feature: OutsideFeature,
): NotifyUnexpectedErrorResource {
    return initNotifyUnexpectedErrorResource(
        initNotifyUnexpectedErrorCoreAction(newNotifyUnexpectedErrorInfra(feature)),
    )
}
