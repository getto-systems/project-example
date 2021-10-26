import { ApplicationAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/action"
import { ObserveBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_observe_field/action"

export interface SearchLoginIDAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly observe: ObserveBoardFieldAction
    clear(): void
}
