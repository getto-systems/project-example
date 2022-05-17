import { env } from "../../../../y_environment/ui/env"

import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { toURL } from "../../../../z_lib/ui/location/init"

import { detectMenuTargetPath } from "../convert"

import { LoadBreadcrumbListShell, LoadMenuShell } from "../action"

type OutsideFeature = LocationOutsideFeature

export function newLoadBreadcrumbListShell(feature: OutsideFeature): LoadBreadcrumbListShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(toURL(feature), env.version),
    }
}

export function newLoadMenuShell(feature: OutsideFeature): LoadMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(toURL(feature), env.version),
    }
}
