import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newCheckAuthTicketAction } from "../../../ticket/check/init/view"
import { newRequestResetTokenAction } from "../../../user/password/reset/request_token/init/resource"
import { newResetPasswordAction } from "../../../user/password/reset/reset/init/view"
import { newAuthenticatePasswordAction } from "../../../user/password/authenticate/init/view"
import { newSignActionShell } from "./shell"

import { initSignAction, SignAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature & LocationOutsideFeature
export function newSignAction(feature: OutsideFeature): SignAction {
    return initSignAction(newSignActionShell(feature), {
        check: () => newCheckAuthTicketAction(feature),

        password_authenticate: () => newAuthenticatePasswordAction(feature),

        password_reset_requestToken: () => newRequestResetTokenAction(feature),
        password_reset: () => newResetPasswordAction(feature),
    })
}
