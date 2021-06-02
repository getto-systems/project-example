import { env } from "../../../../y_environment/_ui/env"

import { newLocationDetecter } from "../../../../../ui/vendor/getto-application/location/init"

import { newCheckDeployExistsRemote } from "../infra/remote/check_deploy_exists"

import { detectApplicationTargetPath } from "./core"

import { FindNextVersionInfra } from "../infra"

import { FindNextVersionLocationDetecter } from "../method"
import { LocationOutsideFeature } from "../../../../../ui/vendor/getto-application/location/infra"

export function newFindNextVersionLocationDetecter(
    feature: LocationOutsideFeature,
): FindNextVersionLocationDetecter {
    return newLocationDetecter(feature, detectApplicationTargetPath({ version: env.version }))
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
