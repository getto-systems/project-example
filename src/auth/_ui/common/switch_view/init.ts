import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/feature"
import { toURL } from "../../../../../ui/vendor/getto-application/location/init"

import { detectSignViewType } from "./core"

import { SignViewDetecter } from "./data"

export function newSignViewLocationDetecter(
    feature: LocationOutsideFeature,
): SignViewDetecter {
    return () => detectSignViewType(toURL(feature))
}
