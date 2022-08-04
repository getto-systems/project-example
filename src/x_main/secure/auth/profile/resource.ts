import { BaseResource } from "../../base/resource"
import { RequestResetTokenAction } from "../../../../auth/user/password/reset/request_token/action"
import { ChangePasswordAction } from "../../../../auth/user/password/change/action"

export type ProfilePageResource = BaseResource &
    Readonly<{
        change: ChangePasswordAction
        requestToken: RequestResetTokenAction
    }>
