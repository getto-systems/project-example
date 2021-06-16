import { env } from "../../../../../y_environment/_ui/env"
import { toURL } from "../../../../../z_details/_ui/location/init"

import { GetScriptPathInfra } from "./infra"

import { GetScriptPathDetecter } from "./method"
import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

import { detectPathname } from "./converter"

export function newGetScriptPathLocationDetecter(
    feature: LocationOutsideFeature,
): GetScriptPathDetecter {
    return () => detectPathname(toURL(feature))
}

export function newGetSecureScriptPathInfra(): GetScriptPathInfra {
    return {
        config: {
            secureServerURL: env.secureServerURL,
        },
    }
}
