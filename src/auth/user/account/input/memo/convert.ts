import { converter } from "../../../../../z_lib/ui/validate/helper"
import { check_text_tooLong } from "../../../../../z_lib/ui/validate/text"

import { AuthUserMemo } from "../../kernel/data"
import { ConvertAuthUserMemoResult } from "./data"

export const authUserMemoBoardConverter: { (value: string): ConvertAuthUserMemoResult } = converter(
    restoreAuthUserMemo,
    [
        // memo には意味的な制限はないが、使用可能な最大文字数は定義しておく
        // api の設定と同期させること
        check_text_tooLong(255),
    ],
)

export function restoreAuthUserMemo(memo: string): AuthUserMemo {
    return memo as AuthUserMemo
}
