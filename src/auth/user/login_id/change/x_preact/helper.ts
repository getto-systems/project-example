import { VNodeContent } from "../../../../../z_vendor/getto-css/preact/common"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { ChangeLoginIdError } from "../data"

export function changeLoginIdError(err: ChangeLoginIdError): readonly VNodeContent[] {
    switch (err.type) {
        case "not-found":
            return ["ユーザーが見つかりませんでした", "一旦リロードしてやり直してください"]

        case "invalid":
            return ["ログインIDが正しくありません"]

        case "already-registered":
            return ["指定したログインIDはすでに登録されています"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりログインID変更に失敗しました`,
                ...reason.detail,
            ])
    }
}