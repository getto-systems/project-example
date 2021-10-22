import { mockObserveBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_observe_field/mock"
import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/init"

import { SearchLoginIDAction } from "./action"

export function mockSearchLoginIDAction(): SearchLoginIDAction {
    const { input, subscriber } = initInputBoardAction()

    return {
        input,
        observe: mockObserveBoardFieldAction(),
        clear: () => null,
        terminate: () => {
            subscriber.terminate()
        },
    }
}
