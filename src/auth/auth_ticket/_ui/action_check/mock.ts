import { mockCheckAuthTicketCoreAction } from "./core/mock"

import { CheckAuthTicketView, CheckAuthTicketResource } from "./resource"

export function mockCheckAuthTicketView(): CheckAuthTicketView {
    return {
        resource: mockCheckAuthTicketResource(),
        terminate: () => null,
    }
}
export function mockCheckAuthTicketResource(): CheckAuthTicketResource {
    return { core: mockCheckAuthTicketCoreAction() }
}
