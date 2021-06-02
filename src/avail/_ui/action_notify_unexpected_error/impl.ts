import { NotifyUnexpectedErrorResource } from "./resource"

import { NotifyUnexpectedErrorCoreAction } from "./core/action"

export function initNotifyUnexpectedErrorResource(
    error: NotifyUnexpectedErrorCoreAction,
): NotifyUnexpectedErrorResource {
    return { error }
}
