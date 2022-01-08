import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newSignViewTypeDetecter } from "../../router/init/init"

import { SignActionShell } from "../action"

type OutsideFeature = LocationOutsideFeature
export function newSignActionShell(feature: OutsideFeature): SignActionShell {
    return {
        detectViewType: newSignViewTypeDetecter(feature),
    }
}
