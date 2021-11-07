import { detectSearchAuthUserAccountFields, updateSearchAuthUserAccountFieldsQuery } from "./convert"

import { SearchAuthUserAccountFieldsDetecter, UpdateSearchAuthUserAccountFieldsQuery } from "./infra"

export function mockSearchAuthUserAccountFieldsDetecter(url: URL): SearchAuthUserAccountFieldsDetecter {
    return (params) => detectSearchAuthUserAccountFields(url, params)
}
export function mockUpdateSearchAuthUserAccountFieldsQuery(
    url: URL,
    updater: { (url: URL): void },
): UpdateSearchAuthUserAccountFieldsQuery {
    return (fields) => {
        updater(updateSearchAuthUserAccountFieldsQuery(url, fields))
    }
}
