import {
    clearFocusAuthUserAccountQuery,
    detectFocusAuthUserAccount,
    detectSearchAuthUserAccountFilter,
    updateFocusAuthUserAccountQuery,
    updateSearchAuthUserAccountFilterQuery,
} from "./query"

import { SearchAuthUserAccountShell } from "../action"

import { DetectFocusListKeyResult } from "../../../../../common/util/list/data"

export function mockSearchAuthUserAccountShell(
    url: URL,
    updater: { (url: URL): void },
): SearchAuthUserAccountShell {
    return {
        detectFilter: () => detectSearchAuthUserAccountFilter(url),
        updateQuery: (fields) => {
            updater(updateSearchAuthUserAccountFilterQuery(url, fields))
        },
        focus: {
            detect: () => detectFocusAuthUserAccount(url),
            update: (data: DetectFocusListKeyResult) => {
                if (data.found) {
                    updater(updateFocusAuthUserAccountQuery(url, data.key))
                } else {
                    updater(clearFocusAuthUserAccountQuery(url))
                }
            },
        },
    }
}
