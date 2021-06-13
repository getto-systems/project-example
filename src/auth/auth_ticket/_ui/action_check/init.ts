import { newCheckAuthTicketInfra } from "../check/init"
import { newStartContinuousRenewAuthnInfoInfra } from "../start_continuous_renew/init"
import { newGetSecureScriptPathInfra } from "../../../_ui/common/secure/get_script_path/init"
import { newGetScriptPathLocationDetecter } from "../../../_ui/common/secure/get_script_path/init"

import { initCheckAuthTicketView } from "./impl"
import { initCheckAuthTicketCoreAction, initCheckAuthTicketCoreMaterial } from "./core/impl"

import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/infra"
import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/infra"

import { CheckAuthTicketView } from "./resource"

type OutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    LocationOutsideFeature
export function newCheckAuthTicketView(feature: OutsideFeature): CheckAuthTicketView {
    return initCheckAuthTicketView(
        initCheckAuthTicketCoreAction(
            initCheckAuthTicketCoreMaterial(
                {
                    check: newCheckAuthTicketInfra(feature),
                    startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                    getSecureScriptPath: newGetSecureScriptPathInfra(),
                },
                newGetScriptPathLocationDetecter(feature),
            ),
        ),
    )
}
