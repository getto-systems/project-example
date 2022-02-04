import { BaseResource } from "../../../../example/action_base/resource"
import { RequestResetTokenProfileAction } from "../../../../auth/user/password/reset/request_token/action"
import { ChangePasswordAction } from "../../../../auth/user/password/change/action"

export type ProfilePageResource = BaseResource &
    Readonly<{
        change: ChangePasswordAction
        requestToken: RequestResetTokenProfileAction
    }>
