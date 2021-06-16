import { initSignLinkResource } from "../../../../_ui/common/nav/action_nav/impl"

import { CheckResetTokenSendingStatusView } from "./resource"

import { CheckResetTokenSendingStatusCoreAction } from "./core/action"

export function initCheckResetTokenSendingStatusView(
    action: CheckResetTokenSendingStatusCoreAction,
): CheckResetTokenSendingStatusView {
    return {
        resource: { checkStatus: action, ...initSignLinkResource() },
        terminate: () => action.terminate(),
    }
}
