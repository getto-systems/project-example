import { RepositoryOutsideFeature } from "../../repository/feature"
import { SearchColumnsInfra } from "./infra"
import { newSearchColumnsRepository } from "./init/repository/columns"

export function newSearchColumnsInfra(
    feature: RepositoryOutsideFeature,
    key: string,
): SearchColumnsInfra {
    return {
        columns: newSearchColumnsRepository(feature, key),
    }
}
