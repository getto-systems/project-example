import { converter } from "../../../../../common/util/validate/helper"
import { check_text_empty, check_text_tooLong } from "../../../../../common/util/validate/text"

import { ValidatePasswordValue, Password } from "./data"

export const passwordConverter: { (value: string): ValidatePasswordValue } = converter(
    markPassword,
    [
        check_text_empty,
        // password には意味的な制限はないが、使用可能な最大文字数は定義しておく
        // api の設定と同期すること
        check_text_tooLong(100),
    ],
)

export function emptyPassword(): Password {
    return markPassword("")
}

function markPassword(password: string): Password {
    return password as Password
}
