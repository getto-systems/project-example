import { env } from "../../../y_environment/ui/env"

import { toURL } from "../../../z_lib/ui/location/init"

import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { MenuTargetPathDetecter } from "./infra"

import { detectMenuTargetPath } from "./convert"

export function newLoadMenuLocationDetecter(feature: LocationOutsideFeature): MenuTargetPathDetecter {
    return () => detectMenuTargetPath(toURL(feature), env.version)
}
