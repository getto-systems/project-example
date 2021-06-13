import { env } from "../../../y_environment/_ui/env"

import { toURL } from "../../../../ui/vendor/getto-application/location/init"

import { LoadMenuDetecter } from "./method"

import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/feature"

import { detectMenuTargetPath } from "./converter"

export function newLoadMenuLocationDetecter(feature: LocationOutsideFeature): LoadMenuDetecter {
    return () => detectMenuTargetPath(toURL(feature), env.version)
}
