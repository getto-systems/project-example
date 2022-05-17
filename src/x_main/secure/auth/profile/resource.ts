import { BaseResource } from "../../../../common/base/resource"
import { RequestResetTokenAction } from "../../../../auth/user/password/reset/request_token/action"
import { ChangePasswordAction } from "../../../../auth/user/password/change/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"

export type ProfilePageResource = BaseResource &
    Readonly<{
        change: Readonly<{
            editable: EditableBoardAction
            change: ChangePasswordAction
        }>
        requestToken: Readonly<{
            editable: EditableBoardAction
            requestToken: RequestResetTokenAction
        }>
    }>
