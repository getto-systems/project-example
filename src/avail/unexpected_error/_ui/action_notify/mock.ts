import { NotifyUnexpectedErrorAction } from "./action"

export function mockNotifyUnexpectedErrorAction(): NotifyUnexpectedErrorAction {
    return {
        notify: () => {
            // mock では特に何もしない
        },
    }
}
