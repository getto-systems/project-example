import { env } from "../../../../y_environment/_ui/env"

import { newDetecter } from "../../../../../ui/vendor/getto-application/location/init"

import { detectMenuTargetPath } from "./detecter"

import { LoadMenuLocationDetecter } from "../method"
import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/infra"

export function newLoadMenuLocationDetecter(
    feature: LocationOutsideFeature,
): LoadMenuLocationDetecter {
    return newDetecter(feature, detectMenuTargetPath({ version: env.version }))
}
