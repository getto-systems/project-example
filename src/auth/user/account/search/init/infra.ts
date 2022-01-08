import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import { newSearchColumnsRepository } from "../../../../../z_lib/ui/search/columns/init/columns_repository"

import { SearchAuthUserAccountInfra } from "../action"
import { newSearchAuthUserAccountRemote } from "./search_remote"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newSearchAuthUserAccountInfra(
    feature: OutsideFeature,
    key: string,
): SearchAuthUserAccountInfra {
    return {
        searchRemote: newSearchAuthUserAccountRemote(feature),
        columnsRepository: newSearchColumnsRepository(feature, key),
    }
}
