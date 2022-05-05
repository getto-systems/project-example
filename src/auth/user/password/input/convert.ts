import { ConvertPasswordResult, Password } from "./data"
import { converter } from "../../../../z_lib/ui/validate/helper"
import { check_text_empty, check_text_tooLong } from "../../../../z_lib/ui/validate/text"

export const passwordBoardConverter: { (value: string): ConvertPasswordResult } = converter(
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
