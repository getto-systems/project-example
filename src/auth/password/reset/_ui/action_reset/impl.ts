import { initSignLinkResource } from "../../../../_ui/common/nav/action_nav/init"

import { ResetPasswordAction, ResetPasswordView } from "./resource"
import { ResetPasswordCoreAction } from "./core/action"
import { ResetPasswordFormAction } from "./form/action"

export function initResetPasswordView(
    actions: Readonly<{
        core: ResetPasswordCoreAction
        form: ResetPasswordFormAction
    }>,
): ResetPasswordView {
    const action = initAction(actions)
    return {
        resource: { reset: action, ...initSignLinkResource() },
        terminate: () => action.terminate(),
    }
}

function initAction(
    actions: Readonly<{
        core: ResetPasswordCoreAction
        form: ResetPasswordFormAction
    }>,
): ResetPasswordAction {
    return {
        ...actions,
        terminate: () => {
            actions.core.terminate()
            actions.form.terminate()
        },
    }
}
