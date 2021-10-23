import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { SearchUserAccountInfra, UpdateSearchUserAccountFieldsQuery } from "./infra"

import { SearchUserAccountEvent } from "./event"

import { SearchUserAccountFields } from "./data"

export interface SearchUserAccountMethod {
    <S>(updater: UpdateSearchUserAccountFieldsQuery, fields: SearchUserAccountFields, post: Post<SearchUserAccountEvent, S>): Promise<S>
}

interface Search {
    (infra: SearchUserAccountInfra): SearchUserAccountMethod
}
export const searchUserAccount: Search = (infra) => async (updater, fields, post) => {
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
