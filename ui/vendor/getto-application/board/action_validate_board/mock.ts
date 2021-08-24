import { ApplicationMockStateAction } from "../../action/mock"

import { initialValidateBoardState, ValidateBoardAction, ValidateBoardActionState } from "./action"

export function mockValidateBoardAction(): ValidateBoardAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<ValidateBoardActionState>
    implements ValidateBoardAction
{
    readonly initialState: ValidateBoardActionState = initialValidateBoardState

    clear(): void {
        return
    }
}
