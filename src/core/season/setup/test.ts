import { setupActionTestRunner } from "../../../z_vendor/getto-application/action/test_helper"

import { markSeason } from "../kernel/test_helper"

import { mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { mockBoardValueStore } from "../../../z_vendor/getto-application/board/input/test_helper"
import { markBoardValue } from "../../../z_vendor/getto-application/board/kernel/mock"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { initSetupSeasonAction } from "./action"

import { SetupSeasonAction } from "./action"
import { initialLoadSeasonState } from "../load/action"

import { SeasonRepository } from "../kernel/infra"
import { BoardValueStore } from "../../../z_vendor/getto-application/board/input/infra"

import { seasonRepositoryConverter, seasonToBoardValue } from "../kernel/convert"
import { convertDB } from "../../../z_lib/ui/repository/init/convert"

import { Season } from "../kernel/data"

describe("SetupSeason", () => {
    test("setup season", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.setupSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("2021.summer"))
            return resource.setupSeason.setup()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-setup" }])
        })
    })

    test("setup season; default", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.setupSeason.subscriber)

        await runner(() => resource.setupSeason.setup()).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-setup" }])
        })
    })

    test("setup season; invalid input", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.setupSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("invalid-season"))
            return resource.setupSeason.setup()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "invalid-season" }])
        })
    })

    test("setup season; invalid year", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.setupSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("2020.summer"))
            return resource.setupSeason.setup()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "invalid-season" }])
        })
    })

    test("setup season; invalid period", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.setupSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("2020.unknown"))
            return resource.setupSeason.setup()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "invalid-season" }])
        })
    })

    test("open", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.setupSeason.subscriber)

        await runner(() => resource.setupSeason.open()).then((stack) => {
            expect(stack).toEqual([{ type: "edit-season" }])
        })
    })

    test("convert season to board value", () => {
        expect(seasonToBoardValue(markSeason({ year: 2021, period: "summer" }))).toEqual(
            "2021.summer",
        )
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
                    seasonRepository,
                    clock,
                },
                config: {
                    manualSetupSeasonExpire: { expire_millisecond: 1000 },
                },
            },
            {
                load: async () => initialLoadSeasonState,
            },
        ),
    }

    const store = {
        season: mockBoardValueStore(),
    }

    resource.setupSeason.season.input.connector.connect(store.season)

    return {
        resource,
        store,
    }
}

function standard_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter(standard_availableSeasons()))
}

function standard_availableSeasons(): readonly Season[] {
    return [{ year: 2021, period: "summer" } as Season]
}
