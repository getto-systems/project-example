import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { BoardFieldValueConverter } from "../../../../ui/vendor/getto-application/board/validate_field/infra"
import { Password, ValidatePasswordError } from "./data"

// bcrypt を想定しているので、72 バイト以上ではいけない
// api の設定と同期すること
export const PASSWORD_MAX_BYTES = 72

type Converter = BoardFieldValueConverter<Password, BoardValue, ValidatePasswordError>
export const passwordBoardConverter: Converter = (value) => {
    if (value.length === 0) {
        return { valid: false, err: EMPTY }
    }

    const byteLength = new TextEncoder().encode(value).byteLength
    if (byteLength > PASSWORD_MAX_BYTES) {
        return { valid: false, err: TOO_LONG(byteLength > value.length) }
    }

    return { valid: true, value: markPassword(value) }
}

const EMPTY: ValidatePasswordError[] = [{ type: "empty" }]
function TOO_LONG(multiByte: boolean): ValidatePasswordError[] {
    return [{ type: "too-long", maxBytes: PASSWORD_MAX_BYTES, multiByte }]
}

function markPassword(password: string): Password {
    return password as Password
}
