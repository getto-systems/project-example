import { mockAuthenticatePasswordFormAction } from "./form/mock"
import { mockAuthenticatePasswordCoreAction } from "./core/mock"

import { AuthenticatePasswordView, AuthenticatePasswordResource } from "./resource"
import { initSignLinkResource } from "../../../_ui/common/nav/action_nav/impl"

export function mockAuthenticatePasswordView(): AuthenticatePasswordView {
    return {
        resource: mockAuthenticatePasswordResource(),
        terminate: () => null,
    }
}
export function mockAuthenticatePasswordResource(): AuthenticatePasswordResource {
    return {
        authenticate: {
            core: mockAuthenticatePasswordCoreAction(),
            form: mockAuthenticatePasswordFormAction(),
            terminate: () => null,
        },
        ...initSignLinkResource(),
    }
}
