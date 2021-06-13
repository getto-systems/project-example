import { newStartContinuousRenewAuthnInfoInfra } from "../../../auth_ticket/_ui/start_continuous_renew/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../_ui/common/secure/get_script_path/init"
import { newAuthenticatePasswordInfra } from "../authenticate/init"

import { initAuthenticatePasswordView } from "./impl"
import { initAuthenticatePasswordFormAction } from "./form/impl"
import {
    initAuthenticatePasswordCoreAction,
    initAuthenticatePasswordCoreMaterial,
} from "./core/impl"

import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/feature"
import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/feature"

import { AuthenticatePasswordView } from "./resource"

export function newAuthenticatePasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): AuthenticatePasswordView {
    return initAuthenticatePasswordView({
        core: initAuthenticatePasswordCoreAction(
            initAuthenticatePasswordCoreMaterial(
                {
                    startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                    getSecureScriptPath: newGetSecureScriptPathInfra(),
                    authenticate: newAuthenticatePasswordInfra(feature),
                },
                newGetScriptPathLocationDetecter(feature),
            ),
        ),
        form: initAuthenticatePasswordFormAction(),
    })
}
