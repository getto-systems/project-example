import { BaseResource } from "../../../../../src/example/action_base/resource"
import { RequestResetTokenProfileAction } from "../../../../../src/auth/user/password/reset/request_token/action"
import { ChangePasswordAction } from "../../../../../src/auth/user/password/change/action"

export type ProfilePageResource = BaseResource &
    Readonly<{
        change: ChangePasswordAction
        requestToken: RequestResetTokenProfileAction
    }>
