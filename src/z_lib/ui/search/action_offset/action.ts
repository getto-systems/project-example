import { ApplicationAction } from "../../../../../ui/vendor/getto-application/action/action"
import { InputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"

export interface SearchOffsetAction extends ApplicationAction {
    readonly input: InputBoardAction
}
