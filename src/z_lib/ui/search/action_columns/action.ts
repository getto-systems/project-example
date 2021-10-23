import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"
import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export interface SearchColumnsAction extends ApplicationStateAction<SearchColumnsState> {
    readonly input: MultipleInputBoardAction
    readonly full: readonly string[]
}

export type SearchColumnsState = Readonly<{ columns: BoardValue[] }>
export const initialSearchColumnsState: SearchColumnsState = { columns: [] }
