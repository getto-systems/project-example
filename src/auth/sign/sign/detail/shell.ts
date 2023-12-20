import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { toURL } from "../../../../common/util/location/detail"

import { detectSignViewType } from "../../router/convert"

import { SignActionShell } from "../action"

type OutsideFeature = LocationOutsideFeature
export function newSignActionShell(feature: OutsideFeature): SignActionShell {
    return {
        detectViewType: () => detectSignViewType(toURL(feature)),
    }
}
