import { detectSearchUserAccountFields, updateSearchUserAccountFieldsQuery } from "./convert"

import { SearchUserAccountFieldsDetecter, UpdateSearchUserAccountFieldsQuery } from "./infra"

export function mockSearchUserAccountFieldsDetecter(url: URL): SearchUserAccountFieldsDetecter {
    return (params) => detectSearchUserAccountFields(url, params)
}
export function mockUpdateSearchUserAccountFieldsQuery(
    url: URL,
    updater: { (url: URL): void },
): UpdateSearchUserAccountFieldsQuery {
    return (fields) => {
        updater(updateSearchUserAccountFieldsQuery(url, fields))
    }
}
