import { env } from "../../../y_environment/_ui/env"

import { newDetecter } from "../../../../ui/vendor/getto-application/location/init"

import { LoadMenuDetecter } from "./method"

import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/infra"

import { detectMenuTargetPath } from "./converter"

export function newLoadMenuLocationDetecter(feature: LocationOutsideFeature): LoadMenuDetecter {
    return newDetecter(feature, (currentURL) => detectMenuTargetPath(currentURL, env.version))
}
