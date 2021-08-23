import { NotifyUnexpectedErrorAction } from "../action"

export function mockNotifyUnexpectedErrorCoreAction(): NotifyUnexpectedErrorAction {
    return {
        notify: () => {
            // mock では特に何もしない
        },
    }
}
