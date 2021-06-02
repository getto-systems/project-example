import { initGetCurrentVersionCoreAction } from "./core/impl"
import { initGetCurrentVersionResource } from "./impl"

import { GetCurrentVersionResource } from "./resource"

describe("GetCurrentVersion", () => {
    test("load current version", () => {
        const { resource } = standard()

        expect(resource.version.getCurrent()).toEqual("1.0.0")
    })
})

function standard() {
    const resource = initResource(standard_version())

    return { resource }
}

function initResource(version: string): GetCurrentVersionResource {
    return initGetCurrentVersionResource(
        initGetCurrentVersionCoreAction({
            version,
        }),
    )
}

function standard_version(): string {
    return "1.0.0"
}
