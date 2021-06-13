import { newStartContinuousRenewAuthnInfoInfra } from "../../../../auth_ticket/_ui/start_continuous_renew/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../../_ui/common/secure/get_script_path/init"
import { newResetPasswordInfra, newResetPasswordLocationDetecter } from "../reset/init"

import { initResetPasswordView } from "./impl"
import { initResetPasswordCoreAction, initResetPasswordCoreMaterial } from "./core/impl"
import { initResetPasswordFormAction } from "./form/impl"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/repository/feature"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/infra"

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
