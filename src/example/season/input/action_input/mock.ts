import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { InputSeasonAction } from "./action"

export function mockInputSeasonAction(): InputSeasonAction {
    const { input, subscriber } = initInputBoardAction()

    return {
        input,
        terminate: () => {
            subscriber.terminate()
        },
    }
}
