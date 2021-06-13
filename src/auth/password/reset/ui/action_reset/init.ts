import { ResetPasswordView } from "./resource"
import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { RepositoryOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/repository/infra"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/infra"
import { initResetPasswordView } from "./impl"
import { initResetPasswordCoreAction, initResetPasswordCoreMaterial } from "./core/impl"
import { newStartContinuousRenewAuthnInfoInfra } from "../../../../auth_ticket/_ui/start_continuous_renew/impl/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../../_ui/common/secure/get_script_path/init"
import { newResetPasswordInfra, newResetPasswordLocationDetecter } from "../reset/impl/init"
import { initResetPasswordFormAction } from "./form/impl"

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
