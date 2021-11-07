import { toSearchColumns } from "./convert"

import { SearchColumnsInfra } from "./infra"

import { LoadSearchColumnsEvent, SaveSearchColumnsEvent } from "./event"

import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export interface LoadSearchColumnsMethod {
    <S>(initial: readonly string[], post: Post<LoadSearchColumnsEvent, S>): Promise<S>
}
interface Load {
    (infra: SearchColumnsInfra): LoadSearchColumnsMethod
}
export const loadSearchColumns: Load = (infra) => async (initial, post) => {
    const { columns } = infra

    const columnsResult = await columns.get()
    if (!columnsResult.success) {
        return post({ type: "repository-error", err: columnsResult.err })
    }
    if (!columnsResult.found) {
        return post({ type: "succeed-to-load", columns: toSearchColumns(initial) })
    }

    return post({ type: "succeed-to-load", columns: columnsResult.value })
}

export interface SaveSearchColumnsMethod {
    <S>(value: readonly BoardValue[], post: Post<SaveSearchColumnsEvent, S>): Promise<S>
}
interface Save {
    (infra: SearchColumnsInfra): SaveSearchColumnsMethod
}
export const saveSearchColumns: Save = (infra) => async (value, post) => {
    const { columns } = infra

    const searchColumns = toSearchColumns(value)

    const columnsResult = await columns.set(searchColumns)
    if (!columnsResult.success) {
        return post({ type: "repository-error", err: columnsResult.err })
    }

    return post({ type: "succeed-to-save", columns: searchColumns })
}

interface Post<E, S> {
    (event: E): S
}
