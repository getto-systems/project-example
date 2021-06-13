import { newCheckAuthTicketInfra } from "../check/init"
import { newStartContinuousRenewAuthnInfoInfra } from "../start_continuous_renew/init"
import { newGetSecureScriptPathInfra } from "../../../_ui/common/secure/get_script_path/init"
import { newGetScriptPathLocationDetecter } from "../../../_ui/common/secure/get_script_path/init"

import { initCheckAuthTicketView } from "./impl"
import { initCheckAuthTicketCoreAction, initCheckAuthTicketCoreMaterial } from "./core/impl"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

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
