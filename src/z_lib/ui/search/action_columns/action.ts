import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"

import { LoadSearchColumnsMethod, SaveSearchColumnsMethod } from "../columns/method"

import { LoadSearchColumnsEvent, SaveSearchColumnsEvent } from "../columns/event"

export interface SearchColumnsAction extends ApplicationStateAction<SearchColumnsState> {
    readonly input: MultipleInputBoardAction

    load(initial: readonly string[]): Promise<SearchColumnsState>
}

export type SearchColumnsMaterial = Readonly<{
    load: LoadSearchColumnsMethod
    save: SaveSearchColumnsMethod
}>

export type SearchColumnsState =
    | Readonly<{ type: "initial-search" }>
    | LoadSearchColumnsEvent
    | SaveSearchColumnsEvent
export const initialSearchColumnsState: SearchColumnsState = { type: "initial-search" }
