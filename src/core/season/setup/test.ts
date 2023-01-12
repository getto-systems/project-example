import { test, expect } from "vitest"
import { observeApplicationState } from "../../../z_vendor/getto-application/action/test_helper"

import { mockClock, mockClockPubSub } from "../../../common/util/clock/mock"
import { mockBoardValueStore } from "../../../z_vendor/getto-application/board/input/test_helper"
import { initMemoryDB } from "../../../common/util/repository/init/memory"
import { markSeason } from "../kernel/test_helper"

import { initSetupSeasonAction } from "./action"

import { SetupSeasonAction } from "./action"

import { SeasonRepository } from "../kernel/infra"
import { BoardValueStore } from "../../../z_vendor/getto-application/board/input/infra"

import { seasonRepositoryConverter } from "../kernel/convert"
import { convertDB } from "../../../common/util/repository/init/convert"

import { Season } from "../kernel/data"

test("setup season", async () => {
    const { setupSeason, store } = standard()

    expect(
        await observeApplicationState(setupSeason.state, async () => {
            store.season.set("2021.summer")

            return setupSeason.setup()
        }),
    ).toEqual([{ type: "success" }, { type: "initial" }])
})

test("setup season; default", async () => {
    const { setupSeason } = standard()

    expect(
        await observeApplicationState(setupSeason.state, async () => {
            return setupSeason.setup()
        }),
    ).toEqual([{ type: "success" }, { type: "initial" }])
})

function standard() {
    return initResource(standard_season())
}

function initResource(seasonRepository: SeasonRepository): Readonly<{
    setupSeason: SetupSeasonAction
    store: Readonly<{ season: BoardValueStore }>
}> {
    const clock = mockClock(new Date("2021-01-01 10:00:00"), mockClockPubSub())

    const setupSeason = initSetupSeasonAction(
        {
            infra: {
                availableSeasons: standard_availableSeasons(),
                seasonRepository,
                clock,
            },
            config: {
                manualSetupSeasonExpire: { expire_millisecond: 1000 },
                resetToInitialTimeout: { wait_millisecond: 32 },
            },
        },
        {
            state: { ignitionState: Promise.resolve({ type: "initial" }) },
            load: async () => ({ type: "initial" }),
        },
    )

    return {
        setupSeason,
        store: {
            season: mockBoardValueStore(setupSeason.season.input),
        },
    }
}

function standard_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter(standard_availableSeasons()))
}

function standard_availableSeasons(): readonly Season[] {
    return [markSeason({ year: 2021, period: "summer" })]
}
