import { toURL } from "../../../../z_lib/ui/location/init"

import { GetScriptPathShell } from "../infra"

import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { detectPathname } from "../convert"

export function newGetScriptPathShell(feature: LocationOutsideFeature): GetScriptPathShell {
    return {
        detectLocationPathname: () => detectPathname(toURL(feature)),
    }
}


