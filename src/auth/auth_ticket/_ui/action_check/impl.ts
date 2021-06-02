import { CheckAuthTicketView } from "./resource"
import { CheckAuthTicketCoreAction } from "./core/action"

export function initCheckAuthTicketView(
    action: CheckAuthTicketCoreAction,
): CheckAuthTicketView {
    return {
        resource: { core: action },
        terminate: () => action.terminate(),
    }
}
