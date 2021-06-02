import { NotifyUnexpectedErrorCoreAction } from "./action"

export function mockNotifyUnexpectedErrorCoreAction(): NotifyUnexpectedErrorCoreAction {
    return {
        notify: () => {
            // mock では特に何もしない
        },
    }
}
