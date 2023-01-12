import { initTextFieldAction, TextFieldAction } from "../../../../../common/util/input/field/text"
import {
    initMultipleFieldAction,
    MultipleFieldAction,
} from "../../../../../common/util/input/field/multiple"

import { authUserTextConverter, AuthUserTextField } from "./convert"

import { TypeAuthUser } from "../../kernel/data"
import { AuthPermission } from "../../../kernel/data"

export type AuthUserTextFieldAction<K extends AuthUserTextField> = TextFieldAction<TypeAuthUser<K>>

export function initAuthUserTextFieldAction<K extends AuthUserTextField>(
    key: K,
): AuthUserTextFieldAction<K> {
    return initTextFieldAction({ convert: authUserTextConverter(key) })
}

// TODO ticket の下に移動したい
export type AuthPermissionGrantedFieldAction = MultipleFieldAction<AuthPermission>

export function initAuthPermissionGrantedFieldAction(): Readonly<{
    input: AuthPermissionGrantedFieldAction
    setOptions: { (state: readonly AuthPermission[]): void }
}> {
    return initMultipleFieldAction({
        convert: (data) => data,
    })
}
