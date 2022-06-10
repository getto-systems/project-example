import { loginIdBoardConverter } from "./convert"

import { LoginId } from "../kernel/data"
import { initTextFieldAction, TextFieldAction } from "../../../../z_lib/ui/input/field/text"

export type LoginIdFieldAction = TextFieldAction<LoginId>

export function initLoginIdFieldAction(): LoginIdFieldAction {
    return initTextFieldAction({ convert: loginIdBoardConverter })
}
