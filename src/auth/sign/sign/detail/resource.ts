import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { newCheckAuthTicketAction } from "../../../ticket/authenticate/detail/view"
import { newRequestResetTokenAction } from "../../../user/password/reset/request_token/detail/resource"
import { newResetPasswordAction } from "../../../user/password/reset/reset/detail/resource"
import { newAuthenticatePasswordAction } from "../../../user/password/authenticate/detail/resource"
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
