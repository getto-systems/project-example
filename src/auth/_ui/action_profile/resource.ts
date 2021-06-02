import { BaseTypes } from "../../../example/_ui/action_base/resource"
import { LogoutResource } from "../../auth_ticket/_ui/action_logout/resource"

type ProfileTypes = BaseTypes<Resource>
type Resource = LogoutResource
export type ProfileView = ProfileTypes["view"]
export type ProfileResource = ProfileTypes["resource"]
