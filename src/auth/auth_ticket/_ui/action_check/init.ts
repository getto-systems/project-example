import { newCheckAuthTicketInfra } from "../check/init"
import { newStartContinuousRenewAuthnInfoInfra } from "../start_continuous_renew/init"
import { newGetSecureScriptPathInfra } from "../../../_ui/common/secure/get_script_path/init"

import { initCheckAuthTicketView } from "./impl"
import { initCheckAuthTicketCoreAction, initCheckAuthTicketCoreMaterial } from "./core/impl"

import { CheckAuthTicketView } from "./resource"
import { newGetScriptPathLocationDetecter } from "../../../_ui/common/secure/get_script_path/init"
import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/infra"
import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/infra"
import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/infra"

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
