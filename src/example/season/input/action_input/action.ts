import { ApplicationAction } from "../../../../../ui/vendor/getto-application/action/action"
import { InputBoardAction } from "../../../../../ui/vendor/getto-application/board/input/action"

export interface InputSeasonAction extends ApplicationAction {
    readonly input: InputBoardAction
}
