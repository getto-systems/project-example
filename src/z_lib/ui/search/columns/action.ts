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

    get(): readonly string[]
}

export type SearchColumnsInfra = Readonly<{
    columnsRepository: SearchColumnsRepository
}>

export type SearchColumnsState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

const initialState: SearchColumnsState = { type: "initial" }

export function initSearchColumnsAction(infra: SearchColumnsInfra): SearchColumnsAction {
    const { state, post } = initApplicationState({
        initialState,
        ignite: load,
    })

    const { input, store, subscriber } = initMultipleInputBoardAction()

    subscriber.subscribe(() => {
        save(store.get())
    })

    return {
        state,

        input,

        get(): readonly string[] {
            return store.get()
        },
    }

    async function save(columns: readonly string[]): Promise<SearchColumnsState> {
        const { columnsRepository } = infra
        const result = await columnsRepository.set(columns)
        if (!result.success) {
            return post({ type: "repository-error", err: result.err })
        }
        return post({ type: "success" })
    }

    async function load(): Promise<SearchColumnsState> {
        const { columnsRepository } = infra

        const columnsResult = await columnsRepository.get()
        if (!columnsResult.success) {
            return post({ type: "repository-error", err: columnsResult.err })
        }
        if (!columnsResult.found) {
            return post(state.currentState())
        }

        store.set(columnsResult.value)
        return post({ type: "success" })
    }
}
