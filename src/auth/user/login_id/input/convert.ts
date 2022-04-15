import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginId, ValidateLoginIdError } from "./data"
import { ConvertBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"

// login id には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期させること
export const LOGIN_ID_MAX_LENGTH = 100

export function loginIdBoardConverter(
    value: BoardValue,
): ConvertBoardFieldResult<LoginId, ValidateLoginIdError> {
    if (value.length === 0) {
        return { valid: false, err: EMPTY }
    }
    if (value.length > LOGIN_ID_MAX_LENGTH) {
        return { valid: false, err: TOO_LONG }
    }
    return { valid: true, value: value as string as LoginId }
}

const EMPTY: readonly ValidateLoginIdError[] = [{ type: "empty" }]
const TOO_LONG: readonly ValidateLoginIdError[] = [
    { type: "too-long", maxLength: LOGIN_ID_MAX_LENGTH },
]

export function restoreLoginId(loginId: string): LoginId {
    return loginId as LoginId
}
