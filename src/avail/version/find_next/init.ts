import { env } from "../../../y_environment/_ui/env"

import { newDetecter } from "../../../../ui/vendor/getto-application/location/init"

import { newCheckDeployExistsRemote } from "./infra/remote/check_deploy_exists"

import { FindNextVersionDetecter } from "./method"

import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/infra"

import { FindNextVersionInfra } from "./infra"

import { detectApplicationTargetPath } from "./converter"

export function newFindNextVersionLocationDetecter(
    feature: LocationOutsideFeature,
): FindNextVersionDetecter {
    return newDetecter(feature, (currentURL) =>
        detectApplicationTargetPath(currentURL, env.version),
    )
}

export function newFindNextVersionInfra(): FindNextVersionInfra {
    return {
        version: env.version,
        versionSuffix: env.versionSuffix,
        check: newCheckDeployExistsRemote(),
        config: {
            takeLongtimeThreshold: { delay_millisecond: 300 },
        },
    }
}
