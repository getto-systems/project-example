import { AuthUserMemo } from "../../kernel/data"
import { ValidateAuthUserMemoError } from "./data"
import { ConvertBoardFieldResult } from "../../../../../z_vendor/getto-application/board/validate_field/data"

// memo には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期させること
export const MEMO_MAX_LENGTH = 255

export function authUserMemoBoardConverter(
    value: string,
): ConvertBoardFieldResult<AuthUserMemo, ValidateAuthUserMemoError> {
    if (value.length > MEMO_MAX_LENGTH) {
        return { valid: false, err: TOO_LONG }
    }
    return { valid: true, value: value as string as AuthUserMemo }
}

const TOO_LONG: readonly ValidateAuthUserMemoError[] = [
    { type: "too-long", maxLength: MEMO_MAX_LENGTH },
]

export function restoreAuthUserMemo(memo: string): AuthUserMemo {
    return memo as AuthUserMemo
}
