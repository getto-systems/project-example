import { LocationOutsideFeature } from "../../../z_details/_ui/location/feature"
import { toURL } from "../../../z_details/_ui/location/init"

import { detectSignViewType } from "./convert"

import { SignViewDetecter } from "./data"

export function newSignViewLocationDetecter(
    feature: LocationOutsideFeature,
): SignViewDetecter {
    return () => detectSignViewType(toURL(feature))
}
