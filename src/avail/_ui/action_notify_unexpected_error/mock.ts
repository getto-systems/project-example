import { mockNotifyUnexpectedErrorCoreAction } from "./core/mock"

import { NotifyUnexpectedErrorResource } from "./resource"

export function mockNotifyUnexpectedErrorResource(): NotifyUnexpectedErrorResource {
    return {
        error: mockNotifyUnexpectedErrorCoreAction(),
    }
}
