import { mockLogoutAction } from "../../auth_ticket/_ui/action_logout/mock"

import { ProfileResource } from "./resource"
import { mockBaseResource } from "../../../example/_ui/action_base/mock"

export function mockAuthProfileResource(): ProfileResource {
    return {
        ...mockBaseResource(),
        logout: mockLogoutAction(),
    }
}
