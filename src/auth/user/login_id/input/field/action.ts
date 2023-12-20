import { loginIdConverter } from "./convert"

import { initTextFieldBoard, TextFieldBoard } from "../../../../../common/util/board/field/action"
import { BoardInitializer } from "../../../../../common/util/board/input/action"

import { ValidateTextError } from "../../../../../common/util/validate/data"
import { LoginId } from "../../kernel/data"

export type LoginIdField = TextFieldBoard<LoginId, readonly ValidateTextError[]>

export function initLoginIdField(): [LoginIdField, BoardInitializer<LoginId>] {
    return initTextFieldBoard({ convert: loginIdConverter })
}
