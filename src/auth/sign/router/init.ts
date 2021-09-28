import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"
import { toURL } from "../../../z_lib/ui/location/init"

import { detectSignViewType } from "./convert"

import { SignViewDetecter } from "./data"

export function newSignViewLocationDetecter(
    feature: LocationOutsideFeature,
): SignViewDetecter {
    return () => detectSignViewType(toURL(feature))
}
