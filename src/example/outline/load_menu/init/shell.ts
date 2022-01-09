import { env } from "../../../../y_environment/ui/env"

import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { toURL } from "../../../../z_lib/ui/location/init"

import { detectMenuTargetPath } from "../../kernel/convert"

import { LoadMenuShell } from "../action"

type OutsideFeature = LocationOutsideFeature
export function newLoadMenuShell(feature: OutsideFeature): LoadMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(toURL(feature), env.version),
    }
}
