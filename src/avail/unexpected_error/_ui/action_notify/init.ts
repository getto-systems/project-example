import { notifyUnexpectedError } from "../notify/method"

import { NotifyUnexpectedErrorAction } from "./action"

import { NotifyUnexpectedErrorInfra } from "../notify/infra"

export function initNotifyUnexpectedErrorAction(
    infra: NotifyUnexpectedErrorInfra,
): NotifyUnexpectedErrorAction {
    return {
        notify: notifyUnexpectedError(infra),
    }
}
