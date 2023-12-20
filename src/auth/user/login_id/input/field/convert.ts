import { converter } from "../../../../../common/util/validate/helper"
import { check_text_empty, check_text_tooLong } from "../../../../../common/util/validate/text"
import { restoreLoginId } from "../../kernel/convert"

import { ValidateLoginIdValue } from "./data"

export const loginIdConverter: (value: string) => ValidateLoginIdValue = converter(restoreLoginId, [
    check_text_empty,
    // login id には技術的な制限はないが、使用可能な最大文字数は定義しておく
    // api の設定と同期させること
    check_text_tooLong(100),
])
