import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/infra"
import { newDetecter } from "../../../../../ui/vendor/getto-application/location/init"

import { detectSignViewType } from "./core"

import { SignViewLocationDetecter } from "./data"

export function newSignViewLocationDetecter(
    feature: LocationOutsideFeature,
): SignViewLocationDetecter {
    return newDetecter(feature, detectSignViewType)
}
