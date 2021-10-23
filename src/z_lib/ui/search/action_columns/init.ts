import { initMultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { initialSearchColumnsState, SearchColumnsAction, SearchColumnsState } from "./action"

import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"

export function initSearchColumnsAction(full: readonly string[], initial: BoardValue[]): SearchColumnsAction {
    return new Action(full, initial)
}

class Action
    extends ApplicationAbstractStateAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialSearchColumnsState

    readonly input: MultipleInputBoardAction
    readonly full: readonly string[]

    constructor(full: readonly string[], initial: BoardValue[]) {
        super()

        const { input, store, subscriber } = initMultipleInputBoardAction()

        store.set(initial)

        this.input = input
        this.full = full

        subscriber.subscribe(() => {
            this.post({ columns: store.get() })
        })

        this.terminateHook(() => {
            subscriber.terminate()
        })
    }
}
