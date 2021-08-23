import { BaseOutsideFeature, newBaseResource } from "../../../../example/_ui/action_base/init"
import { newLogoutResource } from "../../../auth_ticket/_ui/action_logout/init/resource"

import { initProfileView } from "../init"

import { ProfileView } from "../resource"

export function newProfileView(feature: BaseOutsideFeature): ProfileView {
    return initProfileView({
        ...newBaseResource(feature),
        ...newLogoutResource(feature),
    })
}
