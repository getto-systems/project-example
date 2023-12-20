import { Atom, initAtom } from "../../../../z_vendor/getto-atom/atom"
import { loadState_loaded } from "../../load/data"
import { MultipleFilterBoard, initMultipleFilterBoard } from "../../board/filter/action"

import { SearchColumnsRepository } from "./infra"

import { RepositoryError } from "../../repository/data"
import { TableDataCell } from "../../../../z_vendor/getto-table/preact/core"

export type SearchColumnsBoard = MultipleFilterBoard<TableDataCell, string> &
    Readonly<{
        state: Atom<SearchColumnsState>
    }>

export type SearchColumnsState =
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

export type SearchColumnsInfra = Readonly<{
    columnsRepository: SearchColumnsRepository
}>

export function initSearchColumnsBoard(
    infra: SearchColumnsInfra,
    columns: readonly TableDataCell[],
): SearchColumnsBoard {
    const [board, initializer] = initMultipleFilterBoard({
        initial: columns.filter((column) => column.isInitiallyVisible).map((column) => column.key),
        options: initAtom({ initialState: loadState_loaded(columns) }).state,
        toFilter: (option) => option.key,
        toValue: (option) => option.key,
    })

    const { state, post } = initAtom<SearchColumnsState>({
        initialState: { type: "success" },
        ignite: load,
    })

    board.value.subscribe((value) => {
        save(value)
    })

    return {
        ...board,
        state,
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
        if (columnsResult.found) {
            initializer.init(
                columns
                    .filter((column) => columnsResult.value.includes(column.key))
                    .map((column) => column.key),
            )
        }
        return post({ type: "success" })
    }
}
