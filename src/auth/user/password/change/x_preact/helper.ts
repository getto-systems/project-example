import { PreactContent } from "../../../../../z_vendor/getto-css/preact/common"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"

import { ChangePasswordError } from "../data"

export function changePasswordError(err: ChangePasswordError): readonly PreactContent[] {
    switch (err.type) {
        case "invalid-password":
            return ["現在のパスワードが違います"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりパスワード変更に失敗しました`,
                ...reason.detail,
            ])
    }
}
