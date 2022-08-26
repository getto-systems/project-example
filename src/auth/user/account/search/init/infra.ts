import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { SearchAuthUserAccountInfra } from "../action"
import { newSearchAuthUserAccountRemote } from "./search_remote"

type OutsideFeature = RemoteOutsideFeature
export function newSearchAuthUserAccountInfra(feature: OutsideFeature): SearchAuthUserAccountInfra {
    return {
        searchRemote: newSearchAuthUserAccountRemote(feature),
    }
}
