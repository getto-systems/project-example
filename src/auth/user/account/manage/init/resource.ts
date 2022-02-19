import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"

import { newSearchAuthUserAccountConfig } from "../../search/init/config"
import { newSearchAuthUserAccountShell } from "../../search/init/shell"
import { newSearchAuthUserAccountInfra } from "../../search/init/infra"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountAction } from "../../search/action"

// TODO worker 対応
export function newManageUserAccountResource(
    feature: RemoteOutsideFeature &
        RepositoryOutsideFeature &
        LocationOutsideFeature &
        HistoryOutsideFeature,
): Readonly<{ search: SearchAuthUserAccountAction }> {
    return {
        search: initSearchAuthUserAccountAction({
            infra: newSearchAuthUserAccountInfra(feature, "auth/user/account.search"),
            shell: newSearchAuthUserAccountShell(feature),
            config: newSearchAuthUserAccountConfig(),
        }),
    }
}
