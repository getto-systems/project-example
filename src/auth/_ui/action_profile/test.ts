import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"

import { mockAuthProfileResource } from "./mock"

import { initProfileView } from "./impl"

describe("Profile", () => {
    test("terminate: menu", async () => {
        const { view } = standard()

        const runner = setupActionTestRunner(view.resource.menu.subscriber)

        await runner(() => {
            view.terminate()
            return view.resource.menu.ignite()
        }).then((stack) => {
            // no event after terminate
            expect(stack).toEqual([])
        })
    })

    test("terminate: logout", async () => {
        const { view } = standard()

        const runner = setupActionTestRunner(view.resource.logout.subscriber)

        await runner(() => {
            view.terminate()
            return view.resource.logout.submit()
        }).then((stack) => {
            // no event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const view = initView()

    return { view }
}

function initView() {
    return initProfileView(mockAuthProfileResource())
}
