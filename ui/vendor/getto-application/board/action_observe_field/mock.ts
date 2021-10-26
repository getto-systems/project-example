import { ApplicationAbstractStateAction } from "../../action/init"

import {
    initialObserveBoardFieldState,
    ObserveBoardFieldAction,
    ObserveBoardFieldState,
} from "./action"

export function mockObserveBoardFieldAction(): ObserveBoardFieldAction {
    return new Mock()
}

class Mock
    extends ApplicationAbstractStateAction<ObserveBoardFieldState>
    implements ObserveBoardFieldAction
{
    readonly initialState = initialObserveBoardFieldState

    check(): void {
        return
    }
}
