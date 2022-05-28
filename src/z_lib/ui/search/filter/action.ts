import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"
import { ObserveBoardFieldAction } from "../../../../z_vendor/getto-application/board/observe_field/action"
import { initSearchColumnsAction, SearchColumnsAction, SearchColumnsInfra } from "../columns/action"
import { initSearchOffsetAction, SearchOffsetAction } from "../offset/action"

import { nextSort } from "../sort/helper"

import { SearchSort } from "../sort/data"

export interface SearchFilterField {
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export type SearchFilterFields<K extends string> = readonly [K, SearchFilterField][]

export type SearchFilterProps<S, F> = Readonly<{
    observe: ObserveBoardAction
    offset: SearchOffsetAction
    columns: SearchColumnsAction
    filter: SearchFilterAction<S, F>
    clear: { (): void }
}>

export interface SearchFilterAction<S, F> {
    get: { (): SearchFilter<S, F> }
    setSort: { (sort: SearchSort<S>): void }
    search: { (): SearchFilter<S, F> }
    load: { (): SearchFilter<S, F> }
    sort: { (key: S): SearchFilter<S, F> }
}

export type SearchFilter<S, F> = F &
    Readonly<{
        offset: string
        sort: SearchSort<S>
    }>

export function initSearchFilter<K extends string, S, F>(
    infra: SearchColumnsInfra,
    initialFilter: SearchFilter<S, F>,
    fields: SearchFilterFields<K>,
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
        get: (): SearchFilter<S, F> => filter.value,
        set: (newFilter: SearchFilter<S, F>): SearchFilter<S, F> => (filter.value = newFilter),
        setSort: (sort: SearchSort<S>) =>
            filter.set({
                ...filter.value,
                sort,
            }),
        search: (): SearchFilter<S, F> =>
            filter.set({
                ...filter.value,
                offset: offset.reset(),
                ...pin(),
            }),
        load: (): SearchFilter<S, F> =>
            filter.set({
                ...filter.value,
                offset: offset.get(),
            }),
        sort: (key: S): SearchFilter<S, F> =>
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
