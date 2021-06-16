import { newCheckAuthTicketView } from "../../../auth_ticket/_ui/action_check/init"
import { newRequestResetTokenView } from "../../../password/reset/_ui/action_request_token/init/foreground"
import { newResetPasswordView } from "../../../password/reset/_ui/action_reset/init"
import { newCheckPasswordResetSendingStatus } from "../../../password/reset/_ui/action_check_status/init/foreground"
import { newAuthenticatePasswordView } from "../../../password/_ui/action_authenticate/init"
import { newSignViewLocationDetecter } from "../../common/switch_view/init"

import { initSignView } from "../impl"
import { initSignAction } from "../core/impl"

import { SignView } from "../resource"
import { initSignLinkResource } from "../../common/nav/action_nav/impl"

import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

type OutsideFeature = RepositoryOutsideFeature &
    RemoteOutsideFeature &
    LocationOutsideFeature
export function newSignForeground(feature: OutsideFeature): SignView {
    return initSignView(
        initSignAction(newSignViewLocationDetecter(feature), {
            link: () => initSignLinkResource(),

            check: () => newCheckAuthTicketView(feature),

            password_authenticate: () => newAuthenticatePasswordView(feature),

            password_reset_requestToken: () => newRequestResetTokenView(feature),
            password_reset_checkStatus: () => newCheckPasswordResetSendingStatus(feature),
            password_reset: () => newResetPasswordView(feature),
        }),
    )
}
