import { BaseResource } from "../../../../../src/example/action_base/resource"
import { ChangePasswordResource } from "../../../../../src/auth/user/password/action_change/resource"

export type ProfilePageResource = BaseResource & ChangePasswordResource
