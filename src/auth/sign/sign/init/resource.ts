import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { newCheckAuthTicketAction } from "../../../ticket/authenticate/init/view"
import { newRequestResetTokenAction } from "../../../user/password/reset/request_token/init/resource"
import { newResetPasswordAction } from "../../../user/password/reset/reset/init/view"
import { newAuthenticatePasswordAction } from "../../../user/password/authenticate/init/view"
import { newSignActionShell } from "./shell"

import { initSignAction, SignAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature
export function newSignAction(feature: OutsideFeature): SignAction {
    return initSignAction(newSignActionShell(feature), {
        check: () => newCheckAuthTicketAction(feature),

        password_authenticate: () => newAuthenticatePasswordAction(feature),

        password_reset_requestToken: () => newRequestResetTokenAction(),
        password_reset: () => newResetPasswordAction(feature),
    })
}
