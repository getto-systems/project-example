import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { BoardFieldValueConverter } from "../../../../ui/vendor/getto-application/board/validate_field/infra"
import { Password, ValidatePasswordError } from "./data"

// password には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期すること
export const PASSWORD_MAX_LENGTH = 100

type Converter = BoardFieldValueConverter<Password, BoardValue, ValidatePasswordError>
export const passwordBoardConverter: Converter = (value) => {
    if (value.length === 0) {
        return { valid: false, err: EMPTY }
    }
    if (value.length > PASSWORD_MAX_LENGTH) {
        return { valid: false, err: TOO_LONG }
    }
    return { valid: true, value: markPassword(value) }
}

const EMPTY: ValidatePasswordError[] = [{ type: "empty" }]
const TOO_LONG: ValidatePasswordError[] = [{ type: "too-long", maxLength: PASSWORD_MAX_LENGTH }]

function markPassword(password: string): Password {
    return password as Password
}
