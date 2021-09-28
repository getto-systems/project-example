import { env } from "../../../y_environment/ui/env"
import { avail_config } from "../../x_outside_feature/config"

import { toURL } from "../../../z_lib/ui/location/init"

import { newCheckDeployExistsRemote } from "./init/remote/check_deploy_exists"

import { FindNextVersionDetecter } from "./method"

import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { FindNextVersionInfra } from "./infra"

import { detectApplicationTargetPath } from "./convert"

export function newFindNextVersionLocationDetecter(
    feature: LocationOutsideFeature,
): FindNextVersionDetecter {
    return () => detectApplicationTargetPath(toURL(feature), env.version)
}

export function newFindNextVersionInfra(): FindNextVersionInfra {
    return {
        version: env.version,
        versionSuffix: env.versionSuffix,
        check: newCheckDeployExistsRemote(),
        config: {
            takeLongtimeThreshold: avail_config.takeLongtimeThreshold,
        },
    }
}
