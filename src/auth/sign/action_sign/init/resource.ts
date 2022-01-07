import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"

import { newCheckAuthTicketView } from "../../../ticket/check/init/view"
import { newRequestResetTokenView } from "../../../user/password/reset/request_token/init/resource"
import { newResetPasswordView } from "../../../user/password/reset/reset/init/view"
import { newAuthenticatePasswordView } from "../../../user/password/authenticate/init/view"
import { newSignViewLocationDetecter } from "../../router/init"

import { initSignAction } from "../init"

import { SignView } from "../resource"
import { initSignLinkResource } from "../../action_nav/init"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature & LocationOutsideFeature
export function newSignView(feature: OutsideFeature): SignView {
    return toApplicationView(
        initSignAction(newSignViewLocationDetecter(feature), {
            link: () => initSignLinkResource(),

            check: () => newCheckAuthTicketView(feature),

            password_authenticate: () => newAuthenticatePasswordView(feature),

            password_reset_requestToken: () => newRequestResetTokenView(feature),
            password_reset: () => newResetPasswordView(feature),
        }),
    )
}
