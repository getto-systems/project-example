import { initTextFieldAction, TextFieldAction } from "../../../../../z_lib/ui/input/field/text"
import {
    initMultipleFieldAction,
    MultipleFieldAction,
} from "../../../../../z_lib/ui/input/field/multiple"

import { authUserTextConverter, AuthUserTextField } from "./convert"

import { TypeAuthUser } from "../../kernel/data"
import { AuthRole } from "../../../kernel/data"

export type AuthUserTextFieldAction<K extends AuthUserTextField> = TextFieldAction<TypeAuthUser<K>>

export function initAuthUserTextFieldAction<K extends AuthUserTextField>(
    key: K,
): AuthUserTextFieldAction<K> {
    return initTextFieldAction({ convert: authUserTextConverter(key) })
}

export type AuthUserGrantedRolesFieldAction = MultipleFieldAction<AuthRole>

export function initAuthUserGrantedRolesFieldAction(): Readonly<{
    input: MultipleFieldAction<AuthRole>
    setOptions: { (state: readonly AuthRole[]): void }
}> {
    return initMultipleFieldAction({
        convert: (data) => data,
    })
}
