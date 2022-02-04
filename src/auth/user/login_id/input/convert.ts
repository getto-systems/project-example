import { BoardFieldValueConverter } from "../../../../z_vendor/getto-application/board/validate_field/infra"

import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginID, ValidateLoginIDError } from "./data"

// login id には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期させること
export const LOGIN_ID_MAX_LENGTH = 100

type Converter = BoardFieldValueConverter<LoginID, BoardValue, ValidateLoginIDError>
export const loginIDBoardConverter: Converter = (value) => {
    if (value.length === 0) {
        return { valid: false, err: EMPTY }
    }
    if (value.length > LOGIN_ID_MAX_LENGTH) {
        return { valid: false, err: TOO_LONG }
    }
    return { valid: true, value: markLoginID(value) }
}

const EMPTY: readonly ValidateLoginIDError[] = [{ type: "empty" }]
const TOO_LONG: readonly ValidateLoginIDError[] = [
    { type: "too-long", maxLength: LOGIN_ID_MAX_LENGTH },
]

function markLoginID(loginID: string): LoginID {
    return loginID as LoginID
}
