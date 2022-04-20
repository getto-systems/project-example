import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"
import {
    InputBoardAction,
    initMultipleInputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"

import { toSearchColumns } from "./convert"

import { SearchColumnsRepository } from "./infra"
import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { SearchColumns } from "./data"
import { RepositoryError } from "../../repository/data"

export interface SearchColumnsAction extends StatefulApplicationAction<SearchColumnsState> {
    readonly input: InputBoardAction<MultipleBoardValueStore>

    set(columns: readonly string[]): Promise<SearchColumnsState>
}

export type SearchColumnsInfra = Readonly<{
    columnsRepository: SearchColumnsRepository
}>

export type SearchColumnsState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "success"; columns: SearchColumns }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

const initialState: SearchColumnsState = { type: "initial" }

export function initSearchColumnsAction(infra: SearchColumnsInfra): SearchColumnsAction {
    return new Action(infra)
}

class Action
    extends AbstractStatefulApplicationAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialState

    readonly input: InputBoardAction<MultipleBoardValueStore>

    infra: SearchColumnsInfra
    store: MultipleBoardValueStore

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

    async set(columns: readonly string[]): Promise<SearchColumnsState> {
        this.store.set(columns)
        return this.post({ type: "success", columns: toSearchColumns(columns) })
    }

    async save(columns: SearchColumns): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra
        const result = await columnsRepository.set(columns)
        if (!result.success) {
            return this.post({ type: "repository-error", err: result.err })
        }
        return this.post({ type: "success", columns })
    }

    async load(): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra

        const columnsResult = await columnsRepository.get()
        if (!columnsResult.success) {
            return this.post({ type: "repository-error", err: columnsResult.err })
        }
        if (!columnsResult.found) {
            return this.post(this.currentState())
        }

        return this.post({ type: "success", columns: columnsResult.value })
    }
}
