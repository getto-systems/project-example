import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"

import { newCheckAuthTicketView } from "../../../auth_ticket/_ui/action_check/init/resource"
import { newRequestResetTokenView } from "../../../password/reset/action_request_token/init/resource"
import { newResetPasswordView } from "../../../password/reset/action_reset/init/resource"
import { newAuthenticatePasswordView } from "../../../password/action_authenticate/init/resource"
import { newSignViewLocationDetecter } from "../../common/switch_view/init"

import { initSignAction } from "../init"

import { SignView } from "../resource"
import { initSignLinkResource } from "../../common/nav/action_nav/init"

import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

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
