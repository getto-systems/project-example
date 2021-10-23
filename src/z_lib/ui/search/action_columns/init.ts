import { initMultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { initialSearchColumnsState, SearchColumnsAction, SearchColumnsState } from "./action"

import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"

export function initSearchColumnsAction(initial: BoardValue[]): Readonly<{
    input: SearchColumnsAction
    get: { (): BoardValue[] }
}> {
    const input = new Action(initial)

    return {
        input,
        get: input.get,
    }
}

class Action
    extends ApplicationAbstractStateAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialSearchColumnsState

    readonly input: MultipleInputBoardAction

    get: { (): BoardValue[] }

    constructor(initial: BoardValue[]) {
        super()

        const { input, store, subscriber } = initMultipleInputBoardAction()

        store.set(initial)

        this.get = () => store.get()
        this.input = input

        subscriber.subscribe(() => {
            this.post({ columns: store.get() })
        })

        this.terminateHook(() => {
            subscriber.terminate()
        })
    }
}
