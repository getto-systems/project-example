import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"
import { initMultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { initialSearchColumnsState, SearchColumnsAction, SearchColumnsState } from "./action"

export function mockSearchColumnsAction(full: string[]): SearchColumnsAction {
    return new Mock(full)
}

class Mock
    extends ApplicationMockStateAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialSearchColumnsState

    readonly input: MultipleInputBoardAction
    readonly full: readonly string[]

    constructor(full: string[]) {
        super()

        const { input, subscriber } = initMultipleInputBoardAction()

        this.input = input
        this.full = full

        this.terminateHook(() => {
            subscriber.terminate()
        })
    }
}
