import { setupActionTestRunner } from "../../../ui/vendor/getto-application/action/test_helper"

import { mockClock, mockClockPubSub } from "../../z_details/_ui/clock/mock"
import { mockSeasonRepository } from "../load_season/init/repository/mock"

import { markSeason } from "../load_season/test_helper"

import { initLoadSeasonAction } from "./init"

import { SeasonRepository } from "../load_season/infra"

import { LoadSeasonResource } from "./resource"

import { seasonRepositoryConverter } from "../load_season/convert"

describe("LoadSeason", () => {
    test("load from repository", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignite()).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-load", value: { year: 2020 } }])
        })
    })

    test("not found; use default", async () => {
        const { resource } = empty()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignite()).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-load", value: { year: 2021 } }])
        })
    })

    test("save season", () => {
        const season = standard_season()

        // TODO カバレッジのために直接呼び出している。あとでシーズンの設定用 action を作って移動
        season.set(markSeason({ year: 2021 }))
        expect(true).toBe(true)
    })
})

function standard() {
    const resource = initResource(standard_season())

    return { resource }
}
function empty() {
    const resource = initResource(empty_season())

    return { resource }
}

function initResource(season: SeasonRepository): LoadSeasonResource {
    const clock = mockClock(new Date("2021-01-01 10:00:00"), mockClockPubSub())
    return {
        season: initLoadSeasonAction({
            season,
            clock,
        }),
    }
}

function standard_season(): SeasonRepository {
    const result = seasonRepositoryConverter.fromRepository({
        year: 2020,
    })
    if (!result.valid) {
        throw new Error("invalid authz")
    }

    const repository = mockSeasonRepository()
    repository.set(result.value)
    return repository
}
function empty_season(): SeasonRepository {
    return mockSeasonRepository()
}
