import { test, expect } from "vitest"

import { mockSingleBoardStore } from "../../../common/util/board/input/test_helper"
import { mockClock, mockClockPubSub } from "../../../common/util/clock/mock"
import { initMemoryDB } from "../../../common/util/repository/detail/memory"
import { markSeason } from "../kernel/test_helper"

import { initSetupSeasonAction } from "./action"

import { SetupSeasonAction } from "./action"

import { SingleBoardStore } from "../../../common/util/board/input/infra"
import { SeasonRepository } from "../kernel/infra"

import { seasonRepositoryConverter } from "../kernel/convert"
import { convertDB } from "../../../common/util/repository/detail/convert"

import { Season } from "../kernel/data"
import { observeAtom } from "../../../z_vendor/getto-atom/test_helper"

test("setup season", async () => {
    const { setupSeason, store } = standard()

    const result = observeAtom(setupSeason.state)

    store.season.set("2021.summer")

    await setupSeason.setup()

    expect(result()).toEqual([{ type: "success" }, { type: "initial" }])
})

test("setup season; default", async () => {
    const { setupSeason } = standard()

    const result = observeAtom(setupSeason.state)

    await setupSeason.setup()

    expect(result()).toEqual([{ type: "success" }, { type: "initial" }])
})

function standard() {
    return initResource(standard_season())
}

function initResource(seasonRepository: SeasonRepository): Readonly<{
    setupSeason: SetupSeasonAction
    store: Readonly<{ season: SingleBoardStore }>
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
            season: mockSingleBoardStore(setupSeason.season.input),
        },
    }
}

function standard_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter(standard_availableSeasons()))
}

function standard_availableSeasons(): readonly Season[] {
    return [markSeason({ year: 2021, period: "summer" })]
}
