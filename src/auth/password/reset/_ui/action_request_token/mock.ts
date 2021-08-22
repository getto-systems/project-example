import { initSignLinkResource } from "../../../../_ui/common/nav/action_nav/init"

import { mockRequestResetTokenCoreAction } from "./core/mock"
import { mockRequestResetTokenFormAction } from "./form/mock"

import { RequestResetTokenView, RequestResetTokenResource } from "./resource"

export function mockRequestResetTokenView(): RequestResetTokenView {
    return {
        resource: mockRequestResetTokenResource(),
        terminate: () => null,
    }
}
export function mockRequestResetTokenResource(): RequestResetTokenResource {
    return {
        requestToken: {
            core: mockRequestResetTokenCoreAction(),
            form: mockRequestResetTokenFormAction(),
            terminate: () => null,
        },
        ...initSignLinkResource(),
    }
}
