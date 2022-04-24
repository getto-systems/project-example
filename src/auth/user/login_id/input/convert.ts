import { ConvertLoginIdResult } from "./data"
import { LoginId } from "../kernel/data"
import { converter } from "../../../../z_lib/ui/validate/helper"
import { check_text_empty, check_text_tooLong } from "../../../../z_lib/ui/validate/text"

export const loginIdBoardConverter: { (value: string): ConvertLoginIdResult } = converter(
    restoreLoginId,
    [
        check_text_empty,
        // login id には技術的な制限はないが、使用可能な最大文字数は定義しておく
        // api の設定と同期させること
        check_text_tooLong(100),
    ],
)

export function restoreLoginId(loginId: string): LoginId {
    return loginId as LoginId
}
