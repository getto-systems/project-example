import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"

import { initCheckAuthTicketAction, initCheckAuthTicketMaterial } from "../init"
import { newCheckAuthTicketInfra } from "../../check/init"
import { newStartContinuousRenewAuthnInfoInfra } from "../../start_continuous_renew/init"
import { newGetSecureScriptPathInfra } from "../../../_ui/common/secure/get_script_path/init"
import { newGetScriptPathLocationDetecter } from "../../../_ui/common/secure/get_script_path/init"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

import { CheckAuthTicketView } from "../resource"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature
export function newCheckAuthTicketView(feature: OutsideFeature): CheckAuthTicketView {
    return toApplicationView(
        initCheckAuthTicketAction(
            initCheckAuthTicketMaterial({
                check: newCheckAuthTicketInfra(feature),
                startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                getSecureScriptPath: newGetSecureScriptPathInfra(),
            }),
            newGetScriptPathLocationDetecter(feature),
        ),
    )
}
