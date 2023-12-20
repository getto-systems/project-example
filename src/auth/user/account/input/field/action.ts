import { authUserTextConverter, AuthUserTextFieldName } from "./convert"

import { initTextFieldBoard, TextFieldBoard } from "../../../../../common/util/board/field/action"
import { BoardInitializer } from "../../../../../common/util/board/input/action"

import { TypeAuthUser } from "../../kernel/data"
import { ValidateTextError } from "../../../../../common/util/validate/data"

export type AuthUserTextField<K extends AuthUserTextFieldName> = TextFieldBoard<
    TypeAuthUser<K>,
    readonly ValidateTextError[]
>

export function initAuthUserTextField<K extends AuthUserTextFieldName>(
    key: K,
): [AuthUserTextField<K>, BoardInitializer<TypeAuthUser<K>>] {
    return initTextFieldBoard({ convert: authUserTextConverter(key) })
}
