import { env } from "../../../y_environment/_ui/env"

import { toURL } from "../../../z_details/_ui/location/init"

import { LoadMenuDetecter } from "./method"

import { LocationOutsideFeature } from "../../../z_details/_ui/location/feature"

import { detectMenuTargetPath } from "./convert"

export function newLoadMenuLocationDetecter(feature: LocationOutsideFeature): LoadMenuDetecter {
    return () => detectMenuTargetPath(toURL(feature), env.version)
}
