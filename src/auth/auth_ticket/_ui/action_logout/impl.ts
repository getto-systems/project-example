import { LogoutResource } from "./resource"
import { LogoutCoreAction } from "./core/action"

export function initLogoutResource(action: LogoutCoreAction): LogoutResource {
    return { logout: action }
}
