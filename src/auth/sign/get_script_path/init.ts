import { env } from "../../../y_environment/ui/env"
import { toURL } from "../../../z_lib/ui/location/init"

import { GetScriptPathInfra, SecureServerURL } from "./infra"

import { GetScriptPathDetecter } from "./infra"
import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { detectPathname } from "./convert"

export function newGetScriptPathLocationDetecter(
    feature: LocationOutsideFeature,
): GetScriptPathDetecter {
    return () => detectPathname(toURL(feature))
}

export function newGetSecureScriptPathInfra(): GetScriptPathInfra {
    return {
        config: {
            secureServerURL: env.secureServerURL as SecureServerURL,
        },
    }
}
