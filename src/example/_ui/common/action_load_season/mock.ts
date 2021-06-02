import { mockLoadSeasonCoreAction } from "./core/mock"

import { LoadSeasonResource } from "./resource"

export function mockLoadSeasonResource(): LoadSeasonResource {
    return {
        season: mockLoadSeasonCoreAction(),
    }
}
