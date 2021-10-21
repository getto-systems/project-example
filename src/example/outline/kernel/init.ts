import { env } from "../../../y_environment/ui/env"

import { toURL } from "../../../z_lib/ui/location/init"

import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { LoadMenuDetecter } from "./infra"

import { detectMenuTargetPath } from "./convert"

export function newLoadMenuLocationDetecter(feature: LocationOutsideFeature): LoadMenuDetecter {
    return () => detectMenuTargetPath(toURL(feature), env.version)
}
