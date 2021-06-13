import { env } from "../../../y_environment/_ui/env"

import { toURL } from "../../../../ui/vendor/getto-application/location/init"

import { newCheckDeployExistsRemote } from "./infra/remote/check_deploy_exists"

import { FindNextVersionDetecter } from "./method"

import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/feature"

import { FindNextVersionInfra } from "./infra"

import { detectApplicationTargetPath } from "./converter"

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
            takeLongtimeThreshold: { delay_millisecond: 300 },
        },
    }
}
