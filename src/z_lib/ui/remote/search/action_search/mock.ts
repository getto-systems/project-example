import { mockObserveBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_observe_field/mock"
import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/init"

import { SearchOffsetAction } from "./action"

export function mockSearchOffsetAction(): SearchOffsetAction {
    const { input, subscriber } = initInputBoardAction()

    return {
        input,
        observe: mockObserveBoardFieldAction(),
        terminate: () => {
            subscriber.terminate()
        },
    }
}
