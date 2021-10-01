import { toApplicationView } from "../../../../../../../ui/vendor/getto-application/action/helper"

import { newStartContinuousRenewAuthnInfoInfra } from "../../../../../ticket/start_continuous_renew/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../../../sign/get_script_path/init"
import { newResetPasswordInfra, newResetPasswordLocationDetecter } from "../../reset/init"

import { initResetPasswordAction, initResetPasswordMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../../z_lib/ui/location/feature"

import { ResetPasswordView } from "../resource"

export function newResetPasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ResetPasswordView {
    return toApplicationView(
        initResetPasswordAction(
            initResetPasswordMaterial({
                startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                getSecureScriptPath: newGetSecureScriptPathInfra(),
                reset: newResetPasswordInfra(feature),
            }),
            {
                getScriptPath: newGetScriptPathLocationDetecter(feature),
                reset: newResetPasswordLocationDetecter(feature),
            },
        ),
    )
}