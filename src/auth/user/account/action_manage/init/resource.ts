import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"

import {
    initSearchAuthUserAccountAction,
    initSearchAuthUserAccountMaterial,
} from "../../action_search/init"
import {
    newSearchAuthUserAccountFieldsDetecter,
    newSearchAuthUserAccountInfra,
    newUpdateSearchAuthUserAccountFieldsQuery,
} from "../../search/init"

import { ManageUserAccountResource } from "../resource"
import { newSearchColumnsInfra } from "../../../../../z_lib/ui/search/columns/init"

export function newManageUserAccountResource(
    feature: RemoteOutsideFeature &
        RepositoryOutsideFeature &
        LocationOutsideFeature &
        HistoryOutsideFeature,
): ManageUserAccountResource {
    return {
        search: initSearchAuthUserAccountAction(
            initSearchAuthUserAccountMaterial({
                search: newSearchAuthUserAccountInfra(feature),
                columns: newSearchColumnsInfra(feature, "auth/user/account.search")
            }),
            newSearchAuthUserAccountFieldsDetecter(feature),
            newUpdateSearchAuthUserAccountFieldsQuery(feature),
        ),
    }
}
