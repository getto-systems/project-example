import { ApplicationMockStateAction } from "../../action/mock"

import { initialObserveBoardState, ObserveBoardAction, ObserveBoardActionState } from "./action"

export function mockValidateBoardAction(): ObserveBoardAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<ObserveBoardActionState>
    implements ObserveBoardAction
{
    readonly initialState: ObserveBoardActionState = initialObserveBoardState

    clear(): void {
        return
    }
}
