import {
    ApplicationState,
    initApplicationState,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"
import {
    InputBoardAction,
    initMultipleInputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"

import { SearchColumnsRepository } from "./infra"
import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { RepositoryError } from "../../repository/data"

export interface SearchColumnsAction extends StatefulApplicationAction<SearchColumnsState> {
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
    return new Action(infra)
}

class Action implements SearchColumnsAction {
    readonly infra: SearchColumnsInfra
    readonly state: ApplicationState<SearchColumnsState>
    readonly post: (state: SearchColumnsState) => SearchColumnsState

    readonly input: InputBoardAction<MultipleBoardValueStore>

    store: MultipleBoardValueStore

    constructor(infra: SearchColumnsInfra) {
        const { state, post } = initApplicationState({
            initialState,
            ignite: () => this.load(),
        })
        this.state = state
        this.post = post

        const { input, store, subscriber } = initMultipleInputBoardAction()

        this.input = input
        this.store = store
        this.infra = infra

        subscriber.subscribe(() => {
            this.save(store.get())
        })
    }

    get(): readonly string[] {
        return this.store.get()
    }

    async save(columns: readonly string[]): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra
        const result = await columnsRepository.set(columns)
        if (!result.success) {
            return this.post({ type: "repository-error", err: result.err })
        }
        return this.post({ type: "success" })
    }

    async load(): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra

        const columnsResult = await columnsRepository.get()
        if (!columnsResult.success) {
            return this.post({ type: "repository-error", err: columnsResult.err })
        }
        if (!columnsResult.found) {
            return this.post(this.state.currentState())
        }

        this.store.set(columnsResult.value)
        return this.post({ type: "success" })
    }
}
