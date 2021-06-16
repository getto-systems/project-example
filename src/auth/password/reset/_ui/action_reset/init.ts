import { newStartContinuousRenewAuthnInfoInfra } from "../../../../auth_ticket/_ui/start_continuous_renew/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../../_ui/common/secure/get_script_path/init"
import { newResetPasswordInfra, newResetPasswordLocationDetecter } from "../reset/init"

import { initResetPasswordView } from "./impl"
import { initResetPasswordCoreAction, initResetPasswordCoreMaterial } from "./core/impl"
import { initResetPasswordFormAction } from "./form/impl"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

import { ResetPasswordView } from "./resource"

export function newResetPasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ResetPasswordView {
    return initResetPasswordView({
        core: initResetPasswordCoreAction(
            initResetPasswordCoreMaterial(
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
        form: initResetPasswordFormAction(),
    })
}
