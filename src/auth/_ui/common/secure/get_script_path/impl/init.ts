import { env } from "../../../../../../y_environment/env"
import { newLocationDetecter } from "../../../../../../../ui/vendor/getto-application/location/init"

import { detectPathname } from "./core"
import { GetScriptPathInfra } from "../infra"

import { GetScriptPathLocationDetecter } from "../method"
import { LocationOutsideFeature } from "../../../../../../../ui/vendor/getto-application/location/infra"

export function newGetScriptPathLocationDetecter(
    feature: LocationOutsideFeature,
): GetScriptPathLocationDetecter {
    return newLocationDetecter(feature, detectPathname)
}

export function newGetSecureScriptPathInfra(): GetScriptPathInfra {
    return {
        config: {
            secureServerURL: env.secureServerURL,
        },
    }
}
