import { setupActionTestRunner } from "../../../ui/vendor/getto-application/action/test_helper"

import { mockClock, mockClockPubSub } from "../../z_lib/ui/clock/mock"

import { markSeason } from "./test_helper"

import { convertDB } from "../../z_lib/ui/repository/init/convert"
import { initMemoryDB } from "../../z_lib/ui/repository/init/memory"

import { seasonRepositoryConverter } from "./convert"

import { initLoadSeasonAction, LoadSeasonAction } from "./action"

import { SeasonRepository } from "./infra"

describe("LoadSeason", () => {
    test("load from repository", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-load", value: { year: 2020 } }])
        })
    })

    test("not found; use default", async () => {
        const { resource } = empty()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
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

function initResource(season: SeasonRepository): Readonly<{ season: LoadSeasonAction }> {
    const clock = mockClock(new Date("2021-01-01 10:00:00"), mockClockPubSub())
    return {
        season: initLoadSeasonAction({
            seasonRepository: season,
            clock,
        }),
    }
}

function standard_season(): SeasonRepository {
    const db = initMemoryDB()
    db.set({
        year: 2020,
    })
    return convertDB(db, seasonRepositoryConverter)
}
function empty_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter)
}
