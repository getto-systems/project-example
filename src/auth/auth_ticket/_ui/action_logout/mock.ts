import { mockLogoutCoreAction } from "./core/mock"

import { LogoutResource } from "./resource"

export function mockLogoutResource(): LogoutResource {
    return { logout: mockLogoutCoreAction() }
}
