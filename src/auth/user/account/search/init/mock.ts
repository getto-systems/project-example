import {
    detectSearchAuthUserAccountFields,
    updateSearchAuthUserAccountFieldsQuery,
} from "../convert"

import { SearchAuthUserAccountShell } from "../action"

export function mockSearchAuthUserAccountShell(
    url: URL,
    updater: { (url: URL): void },
): SearchAuthUserAccountShell {
    return {
        detectFields: (params) => detectSearchAuthUserAccountFields(url, params),

        updateQuery: (fields) => {
            updater(updateSearchAuthUserAccountFieldsQuery(url, fields))
        },
    }
}
