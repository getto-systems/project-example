import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"

import { markSeason } from "../kernel/test_helper"

import { mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { mockBoardValueStore } from "../../../../ui/vendor/getto-application/board/input/init/mock"
import { markBoardValue } from "../../../../ui/vendor/getto-application/board/kernel/mock"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { initFocusSeasonAction } from "./action"

import { FocusSeasonAction } from "./action"
import { initialLoadSeasonState } from "../load/action"

import { SeasonRepository } from "../kernel/infra"
import { BoardValueStore } from "../../../../ui/vendor/getto-application/board/input/infra"

import { seasonRepositoryConverter, seasonToBoardValue } from "../kernel/convert"
import { convertDB } from "../../../z_lib/ui/repository/init/convert"

describe("FocusSeason", () => {
    test("focus season", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.focusSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("2021.summer"))
            return resource.focusSeason.focus()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-focus" }])
        })
    })

    test("focus season; default", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.focusSeason.subscriber)

        await runner(() => resource.focusSeason.focus()).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-focus" }])
        })
    })

    test("focus season; invalid input", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.focusSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("invalid-season"))
            return resource.focusSeason.focus()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "invalid-season" }])
        })
    })

    test("focus season; invalid year", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.focusSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("2020.summer"))
            return resource.focusSeason.focus()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "invalid-season" }])
        })
    })

    test("focus season; invalid period", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.focusSeason.subscriber)

        await runner(() => {
            store.season.set(markBoardValue("2020.unknown"))
            return resource.focusSeason.focus()
        }).then((stack) => {
            expect(stack).toEqual([{ type: "invalid-season" }])
        })
    })

    test("open", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.focusSeason.subscriber)

        await runner(() => resource.focusSeason.open()).then((stack) => {
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
    resource: Readonly<{ focusSeason: FocusSeasonAction }>
    store: Readonly<{ season: BoardValueStore }>
}> {
    const clock = mockClock(new Date("2021-01-01 10:00:00"), mockClockPubSub())

    const resource = {
        focusSeason: initFocusSeasonAction(
            {
                infra: {
                    seasonRepository,
                    clock,
                },
                config: {
                    focusSeasonExpire: { expire_millisecond: 1000 },
                },
            },
            Promise.resolve(initialLoadSeasonState),
        ),
    }

    const store = {
        season: mockBoardValueStore(),
    }

    resource.focusSeason.season.input.connector.connect(store.season)

    return {
        resource,
        store,
    }
}

function standard_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter)
}
