import { converter } from "../../../../../common/util/validate/helper"
import { check_text_tooLong } from "../../../../../common/util/validate/text"
import { restoreAuthUserField } from "../../kernel/convert"

import { ValidateAuthUserTextValue } from "./data"

export type AuthUserTextFieldName = keyof typeof textValidators
export type AuthUserMultipleFieldName = "granted"

const textValidators = {
    memo: [check_text_tooLong(255)],
} as const

export function authUserTextConverter<K extends AuthUserTextFieldName>(
    key: K,
): (value: string) => ValidateAuthUserTextValue<K> {
    return converter((value: string) => restoreAuthUserField(value), textValidators[key])
}
