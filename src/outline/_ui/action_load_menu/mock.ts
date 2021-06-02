import { mockLoadMenuCoreAction, mockMenu_home } from "./core/mock"

import { LoadMenuResource } from "./resource"

export function mockLoadMenuResource(): LoadMenuResource {
    return {
        menu: mockLoadMenuCoreAction(mockMenu_home()),
    }
}
