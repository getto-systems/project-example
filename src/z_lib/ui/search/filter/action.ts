import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"
import { ObserveBoardFieldAction } from "../../../../z_vendor/getto-application/board/observe_field/action"
import { initSearchColumnsAction, SearchColumnsAction, SearchColumnsInfra } from "../columns/action"
import { initSearchOffsetAction, SearchOffsetAction } from "../offset/action"

import { nextSort } from "../sort/helper"

import { SearchSort } from "../sort/data"

export interface SearchFilterAction {
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export type SearchFilterEntry<K extends string> = [K, SearchFilterAction]

export type SearchFilterProps<S, F> = Readonly<{
    observe: ObserveBoardAction
    offset: SearchOffsetAction
    columns: SearchColumnsAction
    filter: SearchFilter<S, F>
    clear: { (): void }
}>

export interface SearchFilter<S, F> {
    get: { (): SearchFilterValue<S, F> }
    setSort: { (sort: SearchSort<S>): void }
    search: { (): SearchFilterValue<S, F> }
    load: { (): SearchFilterValue<S, F> }
    sort: { (key: S): SearchFilterValue<S, F> }
}

export type SearchFilterValue<S, F> = F &
    Readonly<{
        offset: string
        sort: SearchSort<S>
    }>

export function initSearchFilter<K extends string, S, F>(
    infra: SearchColumnsInfra,
    initialFilter: SearchFilterValue<S, F>,
    fields: readonly SearchFilterEntry<K>[],
    pin: () => F,
): SearchFilterProps<S, F> {
    const offset = initSearchOffsetAction(initialFilter.offset)
    const columns = initSearchColumnsAction(infra)
    const { observe, observeChecker } = initObserveBoardAction({
        fields: fields.map(([key, _]) => key),
    })

    fields.forEach(([key, field]) => {
        field.observe.subscriber.subscribe((result) => {
            observeChecker.update(key, result.hasChanged)
        })
    })

    const clear = () => {
        fields.forEach(([_, field]) => {
            field.clear()
        })
    }

    const filter = {
        value: initialFilter,
        get: (): SearchFilterValue<S, F> => filter.value,
        set: (newFilter: SearchFilterValue<S, F>): SearchFilterValue<S, F> =>
            (filter.value = newFilter),
        setSort: (sort: SearchSort<S>) =>
            filter.set({
                ...filter.value,
                sort,
            }),
        search: (): SearchFilterValue<S, F> =>
            filter.set({
                ...filter.value,
                offset: offset.reset(),
                ...pin(),
            }),
        load: (): SearchFilterValue<S, F> =>
            filter.set({
                ...filter.value,
                offset: offset.get(),
            }),
        sort: (key: S): SearchFilterValue<S, F> =>
            filter.set({
                ...filter.value,
                offset: offset.reset(),
                sort: nextSort(filter.value.sort, key),
            }),
    }

    return {
        observe,
        offset: offset.input,
        columns,
        filter,
        clear,
    }
}
