import { toApplicationView } from "../../../../../../../ui/vendor/getto-application/action/helper"

import { newStartContinuousRenewAuthnInfoInfra } from "../../../../../auth_ticket/_ui/start_continuous_renew/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../../../_ui/common/secure/get_script_path/init"
import { newResetPasswordInfra, newResetPasswordLocationDetecter } from "../../reset/init"

import { initResetPasswordAction, initResetPasswordMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../../z_details/_ui/location/feature"

import { ResetPasswordView } from "../resource"

export function newResetPasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ResetPasswordView {
    return toApplicationView(
        initResetPasswordAction(
            initResetPasswordMaterial(
                {
                    startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                    getSecureScriptPath: newGetSecureScriptPathInfra(),
                    reset: newResetPasswordInfra(feature),
                },
                {
                    getSecureScriptPath: newGetScriptPathLocationDetecter(feature),
                    reset: newResetPasswordLocationDetecter(feature),
                },
            ),
        ),
    )
}
