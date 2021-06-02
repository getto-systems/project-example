import { SignView } from "./resource"

import { SignAction } from "./core/action"

export function initSignView(action: SignAction): SignView {
    return {
        resource: { sign: action },
        terminate: () => action.terminate(),
    }
}
