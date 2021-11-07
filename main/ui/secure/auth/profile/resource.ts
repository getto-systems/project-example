import { BaseResource } from "../../../../../src/example/action_base/resource"
import { ChangePasswordResource } from "../../../../../src/auth/user/password/action_change/resource"
import { RequestResetTokenProfileResource } from "../../../../../src/auth/user/password/reset/action_request_token_profile/resource"

export type ProfilePageResource = BaseResource & ChangePasswordResource & RequestResetTokenProfileResource
