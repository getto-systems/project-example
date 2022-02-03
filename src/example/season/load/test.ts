import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"
import { markSeason } from "../kernel/test_helper"

import { mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { seasonLabel } from "../kernel/helper"

import { initLoadSeasonAction, LoadSeasonAction } from "./action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"

import { seasonRepositoryConverter } from "../kernel/convert"
import { convertDB } from "../../../z_lib/ui/repository/init/convert"

describe("LoadSeason", () => {
    test("load from repository", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "succeed-to-load",
                    season: { year: 2022, period: "summer" },
                    default: false,
                    availableSeasons: [],
                },
            ])
        })
    })

    test("expired; use default", async () => {
        const { resource } = expired()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "succeed-to-load",
                    season: { year: 2021, period: "summer" },
                    default: true,
                    availableSeasons: [],
                },
            ])
        })
    })

    test("not found; use default", async () => {
        const { resource } = empty_summer()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "succeed-to-load",
                    season: { year: 2021, period: "summer" },
                    default: true,
                    availableSeasons: [],
                },
            ])
        })
    })

    test("not found; use default; winter", async () => {
        const { resource } = empty_winter()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "succeed-to-load",
                    season: { year: 2021, period: "winter" },
                    default: true,
                    availableSeasons: [],
                },
            ])
        })
    })

    test("not found; use default; last winter", async () => {
        const { resource } = empty_last_winter()

        const runner = setupActionTestRunner(resource.season.subscriber)

        await runner(() => resource.season.ignitionState).then((stack) => {
            expect(stack).toEqual([
                {
                    type: "succeed-to-load",
                    season: { year: 2021, period: "winter" },
                    default: true,
                    availableSeasons: [
                        { year: 2021, period: "summer" },
                        { year: 2021, period: "winter" },
                    ],
                },
            ])
        })
    })

    test("convert season from repository; before beginning of system", () => {
        const result = seasonRepositoryConverter.fromRepository({
            season: {
                year: 2020,
                period: "summer",
            },
            expires: summer_now().getTime() + 1000,
        })
        expect(result).toEqual({ valid: false })
    })
    test("convert season from repository; invalid period", () => {
        const result = seasonRepositoryConverter.fromRepository({
            season: {
                year: 2021,
                period: "unknown",
            },
            expires: summer_now().getTime() + 1000,
        })
        expect(result).toEqual({ valid: false })
    })
    test("convert season to repository", () => {
        const value = {
            season: markSeason({
                year: 2021,
                period: "summer",
            }),
            expires: summer_now().getTime() + 1000,
        }
        const result = seasonRepositoryConverter.toRepository(value)
        expect(result).toEqual(value)
    })

    test("season label", () => {
        expect(seasonLabel(markSeason({ year: 2021, period: "summer" }))).toEqual("2021 夏")
        expect(seasonLabel(markSeason({ year: 2021, period: "winter" }))).toEqual("2021 冬")
    })
})

function standard() {
    const clock = mockClock(summer_now(), mockClockPubSub())
    const resource = initResource(clock, standard_season())
    return { resource }
}
function expired() {
    const clock = mockClock(summer_now(), mockClockPubSub())
    const resource = initResource(clock, expired_season())
    return { resource }
}
function empty_summer() {
    const clock = mockClock(summer_now(), mockClockPubSub())
    const resource = initResource(clock, empty_season())
    return { resource }
}
function empty_winter() {
    const clock = mockClock(winter_now(), mockClockPubSub())
    const resource = initResource(clock, empty_season())
    return { resource }
}
function empty_last_winter() {
    const clock = mockClock(last_winter_now(), mockClockPubSub())
    const resource = initResource(clock, empty_season())
    return { resource }
}

function initResource(
    clock: Clock,
    season: SeasonRepository,
): Readonly<{ season: LoadSeasonAction }> {
    return {
        season: initLoadSeasonAction({
            season,
            clock,
        }),
    }
}

function summer_now(): Date {
    return new Date("2021-05-01 10:00:00")
}
function winter_now(): Date {
    return new Date("2021-11-01 10:00:00")
}
function last_winter_now(): Date {
    return new Date("2022-01-01 10:00:00")
}

function standard_season(): SeasonRepository {
    const db = initMemoryDB()
    db.set({
        season: {
            year: 2022,
            period: "summer",
        },
        expires: summer_now().getTime() + 1000,
    })
    return convertDB(db, seasonRepositoryConverter)
}
function expired_season(): SeasonRepository {
    const db = initMemoryDB()
    db.set({
        season: {
            year: 2022,
            period: "summer",
        },
        expires: summer_now().getTime() - 1000,
    })
    return convertDB(db, seasonRepositoryConverter)
}
function empty_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter)
}
