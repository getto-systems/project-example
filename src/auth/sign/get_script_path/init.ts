import { env } from "../../../y_environment/ui/env"
import { toURL } from "../../../z_lib/ui/location/init"

import { GetScriptPathConfig, GetScriptPathShell, SecureServerURL } from "./infra"

import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { detectPathname } from "./convert"

export function newGetScriptPathShell(feature: LocationOutsideFeature): GetScriptPathShell {
    return {
        detectLocationPathname: () => detectPathname(toURL(feature)),
    }
}

export function newGetScriptPathConfig(): GetScriptPathConfig {
    return {
        secureServerURL: env.secureServerURL as SecureServerURL,
    }
}
