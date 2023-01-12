import { converter } from "../../../../../common/util/validate/helper"
import { check_text_tooLong } from "../../../../../common/util/validate/text"
import { restoreAuthUserField } from "../../kernel/convert"

import { ValidateAuthUserTextResult } from "./data"

export type AuthUserTextField = keyof typeof textValidators
export type AuthUserMultipleField = "granted"

const textValidators = {
    memo: [check_text_tooLong(255)],
} as const

export function authUserTextConverter<K extends AuthUserTextField>(
    key: K,
): (value: string) => ValidateAuthUserTextResult<K> {
    return converter((value: string) => restoreAuthUserField(value), textValidators[key])
}
