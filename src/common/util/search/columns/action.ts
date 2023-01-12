import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"
import {
    InputBoardAction,
    initMultipleInputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"

import { SearchColumnsRepository } from "./infra"
import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { RepositoryError } from "../../repository/data"

export interface SearchColumnsAction {
    readonly state: ApplicationState<SearchColumnsState>
    readonly input: InputBoardAction<MultipleBoardValueStore>
}

export type SearchColumnsInfra = Readonly<{
    columnsRepository: SearchColumnsRepository
}>

export type SearchColumnsState =
    | Readonly<{ type: "columns"; visibleKeys: readonly string[] }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

export function initSearchColumnsAction(
    infra: SearchColumnsInfra,
    initial: readonly string[],
): SearchColumnsAction {
    const { state, post } = initApplicationState({
        initialState: { type: "columns", visibleKeys: initial },
        ignite: load,
    })

    const { input, store, subscriber } = initMultipleInputBoardAction()

    subscriber.subscribe(() => {
        save(store.get())
    })

    store.set(initial)

    return {
        state,
        input,
    }

    async function save(columns: readonly string[]): Promise<SearchColumnsState> {
        const { columnsRepository } = infra
        const result = await columnsRepository.set(columns)
        if (!result.success) {
            return post({ type: "repository-error", err: result.err })
        }
        return post({ type: "columns", visibleKeys: store.get() })
    }

    async function load(): Promise<SearchColumnsState> {
        const { columnsRepository } = infra

        const columnsResult = await columnsRepository.get()
        if (!columnsResult.success) {
            return post({ type: "repository-error", err: columnsResult.err })
        }
        if (columnsResult.found) {
            store.set(columnsResult.value)
        }
        return post({ type: "columns", visibleKeys: store.get() })
    }
}

export function visibleKeys(state: SearchColumnsState): readonly string[] {
    switch (state.type) {
        case "repository-error":
            return []

        case "columns":
            return state.visibleKeys
    }
}
