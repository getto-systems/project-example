import { newNotifyUnexpectedErrorInfra } from "./infra"

import { initNotifyUnexpectedErrorAction, NotifyUnexpectedErrorAction } from "../action"

export function newNotifyUnexpectedErrorResource(): Readonly<{
    error: NotifyUnexpectedErrorAction
}> {
    return {
        error: initNotifyUnexpectedErrorAction(newNotifyUnexpectedErrorInfra()),
    }
}
