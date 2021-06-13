import { env } from "../../../../../y_environment/_ui/env"
import { newDetecter } from "../../../../../../ui/vendor/getto-application/location/init"

import { GetScriptPathInfra } from "./infra"

import { GetScriptPathDetecter } from "./method"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/infra"

import { detectPathname } from "./converter"

export function newGetScriptPathLocationDetecter(
    feature: LocationOutsideFeature,
): GetScriptPathDetecter {
    return newDetecter(feature, detectPathname)
}

export function newGetSecureScriptPathInfra(): GetScriptPathInfra {
    return {
        config: {
            secureServerURL: env.secureServerURL,
        },
    }
}
