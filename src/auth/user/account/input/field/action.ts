import { initTextFieldAction, TextFieldAction } from "../../../../../z_lib/ui/input/field/text"

import { authUserTextConverter, AuthUserTextField } from "./convert"

import { TypeAuthUser } from "../../kernel/data"

export type AuthUserTextFieldAction<K extends AuthUserTextField> = TextFieldAction<TypeAuthUser<K>>

export function initAuthUserTextFieldAction<K extends AuthUserTextField>(
    key: K,
): AuthUserTextFieldAction<K> {
    return initTextFieldAction({ convert: authUserTextConverter(key) })
}
