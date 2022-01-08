import { initNotifyUnexpectedErrorAction } from "./action"

import { NotifyUnexpectedErrorRemote } from "./infra"

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
    return {
        error: initNotifyUnexpectedErrorAction({
            notify: standard_notify(),
        }),
    }
}

function standard_notify(): NotifyUnexpectedErrorRemote {
    return async () => ({ success: true, value: true })
}
