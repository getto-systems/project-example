import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { SearchAuthUserAccountInfra, UpdateSearchAuthUserAccountFieldsQuery } from "./infra"

import { SearchAuthUserAccountEvent } from "./event"

import { SearchAuthUserAccountFields } from "./data"

export interface SearchAuthUserAccountMethod {
    <S>(updater: UpdateSearchAuthUserAccountFieldsQuery, fields: SearchAuthUserAccountFields, post: Post<SearchAuthUserAccountEvent, S>): Promise<S>
}

interface Search {
    (infra: SearchAuthUserAccountInfra): SearchAuthUserAccountMethod
}
export const searchAuthUserAccount: Search = (infra) => async (updater, fields, post) => {
    updater(fields)
    post({ type: "try-to-search" })

    const { config } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        infra.search(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-search" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-search", err: response.err })
    }

    return post({ type: "succeed-to-search", response: response.value })
}

interface Post<E, S> {
    (event: E): S
}
