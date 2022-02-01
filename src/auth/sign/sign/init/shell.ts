import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { toURL } from "../../../../z_lib/ui/location/init"

import { detectSignViewType } from "../../router/convert"

import { SignActionShell } from "../action"

type OutsideFeature = LocationOutsideFeature
export function newSignActionShell(feature: OutsideFeature): SignActionShell {
    return {
        detectViewType: () => detectSignViewType(toURL(feature)),
    }
}
