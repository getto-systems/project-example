import { passwordConverter } from "./convert"

import { Atom, mapAtom } from "../../../../../z_vendor/getto-atom/atom"
import { BoardInitializer } from "../../../../../common/util/board/input/action"
import { TextFieldBoard, initTextFieldBoard } from "../../../../../common/util/board/field/action"

import { Password } from "./data"
import { ValidateTextError } from "../../../../../common/util/validate/data"

export type PasswordField = TextFieldBoard<Password, readonly ValidateTextError[]> &
    Readonly<{
        character: Atom<PasswordCharacterState>
    }>

export type PasswordCharacterState = Readonly<{ multiByte: boolean }>

export function initPasswordField(): [PasswordField, BoardInitializer<Password>] {
    const [field, initializer] = initTextFieldBoard({
        convert: passwordConverter,
    })
    return [
        {
            ...field,
            character: mapAtom(field.value, (value) => ({
                multiByte: new TextEncoder().encode(value).byteLength > value.length,
            })),
        },
        initializer,
    ]
}
