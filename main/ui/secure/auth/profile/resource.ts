import { BaseResource } from "../../../../../src/example/action_base/resource"
import { ChangePasswordResource } from "../../../../../src/auth/user/password/action_change/resource"
import { RequestResetTokenProfileAction } from "../../../../../src/auth/user/password/reset/request_token/action"

export type ProfilePageResource = BaseResource &
    ChangePasswordResource &
    Readonly<{
        requestToken: RequestResetTokenProfileAction
    }>
