import { newNotifyUnexpectedErrorRemote } from "./notify_remote"

import { NotifyUnexpectedErrorInfra } from "../action"

export function newNotifyUnexpectedErrorInfra(): NotifyUnexpectedErrorInfra {
    return {
        notify: newNotifyUnexpectedErrorRemote(),
    }
}
