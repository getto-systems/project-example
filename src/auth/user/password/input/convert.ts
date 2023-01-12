import { ValidatePasswordResult, Password } from "./data"
import { converter } from "../../../../common/util/validate/helper"
import { check_text_empty, check_text_tooLong } from "../../../../common/util/validate/text"

export const passwordBoardConverter: { (value: string): ValidatePasswordResult } = converter(
    markPassword,
    [
        check_text_empty,
        // password には意味的な制限はないが、使用可能な最大文字数は定義しておく
        // api の設定と同期すること
        check_text_tooLong(100),
    ],
)

function markPassword(password: string): Password {
    return password as Password
}
