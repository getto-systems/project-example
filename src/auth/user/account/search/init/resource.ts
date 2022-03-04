import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"
import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"

import { newSearchAuthUserAccountConfig } from "./config"
import { newSearchAuthUserAccountInfra } from "./infra"
import { newSearchAuthUserAccountShell } from "./shell"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "../action"

type OutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    LocationOutsideFeature &
    HistoryOutsideFeature
export function newSearchAuthUserAccountAction(
    feature: OutsideFeature,
): SearchAuthUserAccountAction {
    return initSearchAuthUserAccountAction({
        infra: newSearchAuthUserAccountInfra(feature),
        shell: newSearchAuthUserAccountShell(feature),
        config: newSearchAuthUserAccountConfig(),
    })
}
