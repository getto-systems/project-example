import { env } from "../../../../y_environment/ui/env"

import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { toURL } from "../../../../common/util/location/init"

import { detectApplicationTargetPath } from "../convert"

import { FindNextVersionShell } from "../action"

type OutsideFeature = LocationOutsideFeature
export function newFindNextVersionShell(feature: OutsideFeature): FindNextVersionShell {
    return {
        detectTargetPath: () => detectApplicationTargetPath(toURL(feature), env.version),
    }
}
