import { notifyUnexpectedError } from "../../notify_unexpected_error/method"

import { NotifyUnexpectedErrorInfra } from "../../notify_unexpected_error/infra"

import { NotifyUnexpectedErrorCoreAction } from "./action"

export function initNotifyUnexpectedErrorCoreAction(
    infra: NotifyUnexpectedErrorInfra,
): NotifyUnexpectedErrorCoreAction {
    return {
        notify: notifyUnexpectedError(infra),
    }
}
