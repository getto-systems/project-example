import { toURL } from "../../../../common/util/location/detail"

import { GetScriptPathShell } from "../infra"

import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { detectPathname } from "../convert"

export function newGetScriptPathShell(feature: LocationOutsideFeature): GetScriptPathShell {
    return {
        detectLocationPathname: () => detectPathname(toURL(feature)),
    }
}
