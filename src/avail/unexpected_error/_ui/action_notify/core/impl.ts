import { notifyUnexpectedError } from "../../notify/method"

import { NotifyUnexpectedErrorInfra } from "../../notify/infra"

import { NotifyUnexpectedErrorCoreAction } from "./action"

export function initNotifyUnexpectedErrorCoreAction(
    infra: NotifyUnexpectedErrorInfra,
): NotifyUnexpectedErrorCoreAction {
    return {
        notify: notifyUnexpectedError(infra),
    }
}
