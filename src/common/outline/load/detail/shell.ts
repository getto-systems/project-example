import { env } from "../../../../y_environment/ui/env"

import { LocationOutsideFeature } from "../../../util/location/feature"

import { toURL } from "../../../util/location/detail"

import { detectMenuTargetPath } from "../convert"

import { OutlineBreadcrumbListShell, OutlineMenuShell } from "../action"

type OutsideFeature = LocationOutsideFeature

export function newOutlineBreadcrumbListShell(feature: OutsideFeature): OutlineBreadcrumbListShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(toURL(feature), env.version),
    }
}

export function newOutlineMenuShell(feature: OutsideFeature): OutlineMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(toURL(feature), env.version),
    }
}
