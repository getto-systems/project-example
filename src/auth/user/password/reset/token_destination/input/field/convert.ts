import { converter } from "../../../../../../../common/util/validate/helper"
import {
    check_text_empty,
    check_text_invalidEmail,
    check_text_tooLong,
} from "../../../../../../../common/util/validate/text"

import { ValidateBoardValue } from "../../../../../../../common/util/board/validate/data"
import { ValidateTextError } from "../../../../../../../common/util/validate/data"
import { ResetTokenDestinationEmail } from "../../kernel/data"
import { restoreResetTokenDestinationEmail } from "../../kernel/convert"

export function resetTokenDestinationEmailConverter(
    value: string,
): ValidateBoardValue<ResetTokenDestinationEmail, readonly ValidateTextError[]> {
    return emailConverter(value)
}

const emailConverter: {
    (value: string): ValidateBoardValue<ResetTokenDestinationEmail, readonly ValidateTextError[]>
} = converter(restoreResetTokenDestinationEmail, [
    check_text_empty,
    // email には技術的な制限はないが、使用可能な最大文字数は定義しておく
    // api の設定と同期させること
    check_text_tooLong(255),
    check_text_invalidEmail,
])
