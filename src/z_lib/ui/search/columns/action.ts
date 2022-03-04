import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"
import {
    MultipleInputBoardAction,
    initMultipleInputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"

import { toSearchColumns } from "./convert"

import { SearchColumnsRepository } from "./infra"
import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { SearchColumns } from "./data"
import { RepositoryError } from "../../repository/data"

export interface SearchColumnsAction extends StatefulApplicationAction<SearchColumnsState> {
    readonly input: MultipleInputBoardAction

    setInitialSearchColumns(initial: readonly string[]): Promise<SearchColumnsState>
}

export type SearchColumnsInfra = Readonly<{
    columnsRepository: SearchColumnsRepository
}>

export type SearchColumnsState =
    | Readonly<{ type: "initial-search" }>
    | Readonly<{ type: "succeed-to-save"; columns: SearchColumns }>
    | Readonly<{ type: "succeed-to-load"; columns: SearchColumns }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

const initialState: SearchColumnsState = { type: "initial-search" }

export function initSearchColumnsAction(infra: SearchColumnsInfra): SearchColumnsAction {
    return new Action(infra)
}

class Action
    extends AbstractStatefulApplicationAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialState

    readonly input: MultipleInputBoardAction

    infra: SearchColumnsInfra
    store: MultipleBoardValueStore

    searchColumns?: SearchColumns

    constructor(infra: SearchColumnsInfra) {
        super({
            ignite: () => this.load(),
            terminate: () => {
                subscriber.terminate()
            },
        })

        const { input, store, subscriber } = initMultipleInputBoardAction()

        this.input = input
        this.store = store
        this.infra = infra

        subscriber.subscribe(() => {
            this.save(toSearchColumns(store.get()))
        })
    }

    async setInitialSearchColumns(initial: readonly string[]): Promise<SearchColumnsState> {
        if (!this.searchColumns) {
            this.searchColumns = toSearchColumns(initial)
        }
        return this.post({ type: "succeed-to-load", columns: this.searchColumns })
    }

    async save(columns: SearchColumns): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra
        const result = await columnsRepository.set(columns)
        if (!result.success) {
            return this.post({ type: "repository-error", err: result.err })
        }
        return this.post({ type: "succeed-to-save", columns })
    }

    async load(): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra

        const columnsResult = await columnsRepository.get()
        if (!columnsResult.success) {
            return this.post({ type: "repository-error", err: columnsResult.err })
        }
        if (!columnsResult.found) {
            return this.post({
                type: "succeed-to-load",
                columns: this.searchColumns || toSearchColumns([]),
            })
        }

        return this.post({ type: "succeed-to-load", columns: columnsResult.value })
    }
}
