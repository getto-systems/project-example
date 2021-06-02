import { initSignLinkResource } from "../../../_ui/common/nav/action_nav/impl"

import { AuthenticatePasswordAction, AuthenticatePasswordView } from "./resource"
import { AuthenticatePasswordCoreAction } from "./core/action"
import { AuthenticatePasswordFormAction } from "./form/action"

export function initAuthenticatePasswordView(
    actions: Readonly<{
        core: AuthenticatePasswordCoreAction
        form: AuthenticatePasswordFormAction
    }>,
): AuthenticatePasswordView {
    const action = initAction(actions)
    return {
        resource: { authenticate: action, ...initSignLinkResource() },
        terminate: () => action.terminate(),
    }
}

function initAction(
    actions: Readonly<{
        core: AuthenticatePasswordCoreAction
        form: AuthenticatePasswordFormAction
    }>,
): AuthenticatePasswordAction {
    return {
        ...actions,
        terminate: () => {
            actions.core.terminate()
            actions.form.terminate()
        },
    }
}
