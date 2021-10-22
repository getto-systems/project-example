import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import {
    HistoryOutsideFeature,
    LocationOutsideFeature,
} from "../../../../../z_lib/ui/location/feature"

import {
    initSearchUserAccountAction,
    initSearchUserAccountMaterial,
} from "../../action_search/init"
import {
    newSearchUserAccountFieldsDetecter,
    newSearchUserAccountInfra,
    newUpdateSearchUserAccountFieldsQuery,
} from "../../search/init"

import { ManageUserAccountView } from "../resource"

export function newManageUserAccountView(
    feature: RemoteOutsideFeature &
        RepositoryOutsideFeature &
        LocationOutsideFeature &
        HistoryOutsideFeature,
): ManageUserAccountView {
    const resource = {
        search: initSearchUserAccountAction(
            initSearchUserAccountMaterial({
                search: newSearchUserAccountInfra(feature),
            }),
            newSearchUserAccountFieldsDetecter(feature),
            newUpdateSearchUserAccountFieldsQuery(feature),
        ),
    }
    return {
        resource,
        terminate: () => {
            resource.search.terminate()
        },
    }
}
