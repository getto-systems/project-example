import { newStartContinuousRenewAuthnInfoInfra } from "../../../auth_ticket/_ui/start_continuous_renew/impl/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../_ui/common/secure/get_script_path/impl/init"
import { newAuthenticatePasswordInfra } from "../authenticate/impl/init"

import { initAuthenticatePasswordView } from "./impl"
import { initAuthenticatePasswordFormAction } from "./form/impl"
import {
    initAuthenticatePasswordCoreAction,
    initAuthenticatePasswordCoreMaterial,
} from "./core/impl"

import { AuthenticatePasswordView } from "./resource"

import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/infra"
import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/infra"
import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/infra"

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
