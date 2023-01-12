import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../common/util/location/feature"
import { RepositoryOutsideFeature } from "../../../../../common/util/repository/feature"

import { newSearchAuthUserAccountConfig } from "./config"
import { newSearchAuthUserAccountInfra } from "./infra"
import { newSearchAuthUserAccountShell } from "./shell"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature & HistoryOutsideFeature
export function newSearchAuthUserAccountAction(
    feature: OutsideFeature,
): SearchAuthUserAccountAction {
    return initSearchAuthUserAccountAction({
        infra: newSearchAuthUserAccountInfra(),
        shell: newSearchAuthUserAccountShell(feature),
        config: newSearchAuthUserAccountConfig(),
    })
}
