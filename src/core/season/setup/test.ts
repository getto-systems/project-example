import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../z_vendor/getto-application/action/test_helper"

import { mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { mockBoardValueStore } from "../../../z_vendor/getto-application/board/input/test_helper"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"
import { markSeason } from "../kernel/test_helper"

import { initSetupSeasonAction } from "./action"

import { SetupSeasonAction } from "./action"

import { SeasonRepository } from "../kernel/infra"
import { BoardValueStore } from "../../../z_vendor/getto-application/board/input/infra"

import { seasonRepositoryConverter } from "../kernel/convert"
import { convertDB } from "../../../z_lib/ui/repository/init/convert"

import { Season } from "../kernel/data"

test("setup season", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.setupSeason.subscriber)

    await runner(() => {
        store.season.set("2021.summer")
        return resource.setupSeason.setup()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "success" }])
    })
})

test("setup season; default", async () => {
    const { resource } = standard()

    const runner = setupActionTestRunner(resource.setupSeason.subscriber)

    await runner(() => resource.setupSeason.setup()).then((stack) => {
        expect(stack).toEqual([{ type: "success" }])
    })
})

function standard() {
    return initResource(standard_season())
}

function initResource(seasonRepository: SeasonRepository): Readonly<{
    resource: Readonly<{ setupSeason: SetupSeasonAction }>
    store: Readonly<{ season: BoardValueStore }>
}> {
    const clock = mockClock(new Date("2021-01-01 10:00:00"), mockClockPubSub())

    const resource = {
        setupSeason: initSetupSeasonAction(
            {
                infra: {
                    availableSeasons: standard_availableSeasons(),
                    seasonRepository,
                    clock,
                },
                config: {
                    manualSetupSeasonExpire: { expire_millisecond: 1000 },
                },
            },
            {
                ignitionState: Promise.resolve({ type: "initial" }),
                load: async () => ({ type: "initial" }),
            },
        ),
    }

    const store = {
        season: mockBoardValueStore(resource.setupSeason.season.input),
    }

    return {
        resource,
        store,
    }
}

function standard_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter(standard_availableSeasons()))
}

function standard_availableSeasons(): readonly Season[] {
    return [markSeason({ year: 2021, period: "summer" })]
}
