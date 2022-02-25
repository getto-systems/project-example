import {
    detectSearchAuthUserAccountFilter,
    updateSearchAuthUserAccountFilterQuery,
} from "../convert"

import { SearchAuthUserAccountShell } from "../action"

export function mockSearchAuthUserAccountShell(
    url: URL,
    updater: { (url: URL): void },
): SearchAuthUserAccountShell {
    return {
        detectFields: () => detectSearchAuthUserAccountFilter(url),

        updateQuery: (fields) => {
            updater(updateSearchAuthUserAccountFilterQuery(url, fields))
        },
    }
}
