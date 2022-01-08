import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"
import { toURL } from "../../../../z_lib/ui/location/init"

import { detectSignViewType } from "../convert"

import { SignViewTypeDetecter } from "../infra"

export function newSignViewTypeDetecter(feature: LocationOutsideFeature): SignViewTypeDetecter {
    return () => detectSignViewType(toURL(feature))
}
