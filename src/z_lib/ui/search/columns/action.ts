import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"

import { SearchColumns } from "./data"
import { RepositoryError } from "../../repository/data"
import { SearchColumnsRepository } from "./infra"
import { toSearchColumns } from "./convert"
import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"
import { MultipleBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"
import { initMultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

export interface SearchColumnsAction extends ApplicationStateAction<SearchColumnsState> {
    readonly input: MultipleInputBoardAction

    load(initial: readonly string[]): Promise<SearchColumnsState>
}

export type SearchColumnsInfra = Readonly<{
    columnsRepository: SearchColumnsRepository
}>

export type SearchColumnsState =
    | Readonly<{ type: "initial-search" }>
    | Readonly<{ type: "succeed-to-save"; columns: SearchColumns }>
    | Readonly<{ type: "succeed-to-load"; columns: SearchColumns }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
export const initialSearchColumnsState: SearchColumnsState = { type: "initial-search" }

export function initSearchColumnsAction(infra: SearchColumnsInfra): SearchColumnsAction {
    return new Action(infra)
}

class Action
    extends ApplicationAbstractStateAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialSearchColumnsState

    readonly input: MultipleInputBoardAction

    infra: SearchColumnsInfra
    store: MultipleBoardValueStore

    constructor(infra: SearchColumnsInfra) {
        super()

        const { input, store, subscriber } = initMultipleInputBoardAction()

        this.input = input
        this.store = store
        this.infra = infra

        subscriber.subscribe(() => {
            this.save(toSearchColumns(store.get()))
        })

        this.terminateHook(() => {
            subscriber.terminate()
        })
    }

    async save(columns: SearchColumns): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra
        const result = await columnsRepository.set(columns)
        if (!result.success) {
            return this.post({ type: "repository-error", err: result.err })
        }
        return this.post({ type: "succeed-to-save", columns })
    }

    async load(initial: readonly string[]): Promise<SearchColumnsState> {
        const { columnsRepository } = this.infra

        const columnsResult = await columnsRepository.get()
        if (!columnsResult.success) {
            return this.post({ type: "repository-error", err: columnsResult.err })
        }
        if (!columnsResult.found) {
            return this.post({ type: "succeed-to-load", columns: toSearchColumns(initial) })
        }

        return this.post({ type: "succeed-to-load", columns: columnsResult.value })
    }
}
