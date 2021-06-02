import { initNotFoundView } from "./impl"
import { initGetCurrentVersionResource } from "../../version/action_get_current/impl"
import { initGetCurrentVersionCoreAction } from "../../version/action_get_current/core/impl"

import { NotFoundView } from "./resource"

describe("NotFound", () => {
    test("terminate", () => {
        const { view } = standard()

        view.terminate()
        expect(true).toBe(true)
    })
})

function standard() {
    const view = initView(standard_version())

    return { view }
}

function initView(version: string): NotFoundView {
    return initNotFoundView(
        initGetCurrentVersionResource(
            initGetCurrentVersionCoreAction({
                version,
            }),
        ),
    )
}

function standard_version(): string {
    return "1.0.0"
}
