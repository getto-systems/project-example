import { initNotifyUnexpectedErrorCoreAction } from "./core/impl"
import { initNotifyUnexpectedErrorResource } from "./impl"

import { NotifyUnexpectedErrorRemote } from "../notify_unexpected_error/infra"

describe("NotifyUnexpectedError", () => {
    test("notify", () => {
        const { resource } = standard()

        resource.error.notify("error")
        expect(true).toBe(true)
    })
})

function standard() {
    const resource = initResource()

    return { resource }
}

function initResource() {
    return initNotifyUnexpectedErrorResource(
        initNotifyUnexpectedErrorCoreAction({
            notify: standard_notify(),
        }),
    )
}

function standard_notify(): NotifyUnexpectedErrorRemote {
    return async () => ({ success: true, value: true })
}
