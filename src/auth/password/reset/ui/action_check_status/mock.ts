import { mockCheckResetTokenSendingStatusCoreAction } from "./core/mock"

import { initSignLinkResource } from "../../../../_ui/common/nav/action_nav/impl"

import {
    CheckResetTokenSendingStatusView,
    CheckResetTokenSendingStatusResource,
} from "./resource"

export function mockCheckResetTokenSendingStatusView(): CheckResetTokenSendingStatusView {
    return {
        resource: mockCheckResetTokenSendingStatusResource(),
        terminate: () => null,
    }
}
export function mockCheckResetTokenSendingStatusResource(): CheckResetTokenSendingStatusResource {
    return {
        checkStatus: mockCheckResetTokenSendingStatusCoreAction(),
        ...initSignLinkResource(),
    }
}
