import { toApplicationView } from "../../../../../../ui/vendor/getto-application/action/helper"

import { newStartContinuousRenewAuthnInfoInfra } from "../../../../ticket/start_continuous_renew/init"
import {
    newGetScriptPathLocationDetecter,
    newGetSecureScriptPathInfra,
} from "../../../../sign/get_script_path/init"
import { newAuthenticatePasswordInfra } from "../../authenticate/init"

import { initAuthenticatePasswordAction, initAuthenticatePasswordMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../z_lib/ui/location/feature"

import { AuthenticatePasswordView } from "../resource"

export function newAuthenticatePasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): AuthenticatePasswordView {
    return toApplicationView(
        initAuthenticatePasswordAction(
            initAuthenticatePasswordMaterial({
                startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                getSecureScriptPath: newGetSecureScriptPathInfra(),
                authenticate: newAuthenticatePasswordInfra(feature),
            }),
            newGetScriptPathLocationDetecter(feature),
        ),
    )
}
