import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"
import { initMultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { initialSearchColumnsState, SearchColumnsAction, SearchColumnsState } from "./action"

export function mockSearchColumnsAction(): SearchColumnsAction {
    return new Mock()
}

class Mock
    extends ApplicationMockStateAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialSearchColumnsState

    readonly input: MultipleInputBoardAction

    constructor() {
        super()

        const { input, subscriber } = initMultipleInputBoardAction()

        this.input = input

        this.terminateHook(() => {
            subscriber.terminate()
        })
    }

    async load(): Promise<SearchColumnsState> {
        return this.initialState
    }
}
