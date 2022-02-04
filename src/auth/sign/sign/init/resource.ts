import { toApplicationView } from "../../../../z_vendor/getto-application/action/helper"
import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newCheckAuthTicketView } from "../../../ticket/check/init/view"
import { newRequestResetTokenView } from "../../../user/password/reset/request_token/init/resource"
import { newResetPasswordView } from "../../../user/password/reset/reset/init/view"
import { newAuthenticatePasswordView } from "../../../user/password/authenticate/init/view"
import { newSignActionShell } from "./shell"

import { initSignAction, SignAction } from "../action"

import { initSignLinkResource } from "../../nav/resource"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature & LocationOutsideFeature
export function newSignView(feature: OutsideFeature): ApplicationView<SignAction> {
    return toApplicationView(
        initSignAction(newSignActionShell(feature), {
            link: () => initSignLinkResource(),

            check: () => newCheckAuthTicketView(feature),

            password_authenticate: () => newAuthenticatePasswordView(feature),

            password_reset_requestToken: () => newRequestResetTokenView(feature),
            password_reset: () => newResetPasswordView(feature),
        }),
    )
}
