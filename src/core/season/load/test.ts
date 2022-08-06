import { test, expect } from "vitest"
import { observeApplicationState } from "../../../z_vendor/getto-application/action/test_helper"
import { markSeason } from "../kernel/test_helper"

import { mockClock, mockClockPubSub } from "../../../z_lib/ui/clock/mock"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { seasonLabel } from "../kernel/helper"

import { initLoadSeasonAction, LoadSeasonAction } from "./action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"

import { seasonRepositoryConverter } from "../kernel/convert"
import { convertDB } from "../../../z_lib/ui/repository/init/convert"

import { Season } from "../kernel/data"
import { currentSeason } from "../kernel/init/current_season"

test("load from repository", async () => {
    const { season } = standard()

    expect(
        await observeApplicationState(season.state, async () => {
            return season.state.ignitionState
        }),
    ).toEqual([
        {
            type: "success",
            season: { year: 2022, period: "summer" },
            default: false,
            availableSeasons: [
                { year: 2022, period: "summer" },
                { year: 2021, period: "winter" },
                { year: 2021, period: "summer" },
            ],
        },
    ])
})

test("expired; use default", async () => {
    const { season } = expired()

    expect(
        await observeApplicationState(season.state, async () => {
            return season.state.ignitionState
        }),
    ).toEqual([
        {
            type: "success",
            season: { year: 2021, period: "summer" },
            default: true,
            availableSeasons: [
                { year: 2022, period: "summer" },
                { year: 2021, period: "winter" },
                { year: 2021, period: "summer" },
            ],
        },
    ])
})

test("not found; use default", async () => {
    const { season } = empty_summer()

    expect(
        await observeApplicationState(season.state, async () => {
            return season.state.ignitionState
        }),
    ).toEqual([
        {
            type: "success",
            season: { year: 2021, period: "summer" },
            default: true,
            availableSeasons: [
                { year: 2022, period: "summer" },
                { year: 2021, period: "winter" },
                { year: 2021, period: "summer" },
            ],
        },
    ])
})

test("not found; use default; winter", async () => {
    const { season } = empty_winter()

    expect(
        await observeApplicationState(season.state, async () => {
            return season.state.ignitionState
        }),
    ).toEqual([
        {
            type: "success",
            season: { year: 2021, period: "winter" },
            default: true,
            availableSeasons: [
                { year: 2022, period: "summer" },
                { year: 2021, period: "winter" },
                { year: 2021, period: "summer" },
            ],
        },
    ])
})

test("not found; use default; last winter", async () => {
    const { season } = empty_last_winter()

    expect(
        await observeApplicationState(season.state, async () => {
            return season.state.ignitionState
        }),
    ).toEqual([
        {
            type: "success",
            season: { year: 2021, period: "winter" },
            default: true,
            availableSeasons: [
                { year: 2022, period: "summer" },
                { year: 2021, period: "winter" },
                { year: 2021, period: "summer" },
            ],
        },
    ])
})

test("season label", () => {
    expect(seasonLabel(markSeason({ year: 2021, period: "summer" }))).toEqual("2021 夏")
    expect(seasonLabel(markSeason({ year: 2021, period: "winter" }))).toEqual("2021 冬")
})

function standard() {
    const clock = mockClock(summer_now(), mockClockPubSub())
    return initResource(clock, standard_season())
}
function expired() {
    const clock = mockClock(summer_now(), mockClockPubSub())
    return initResource(clock, expired_season())
}
function empty_summer() {
    const clock = mockClock(summer_now(), mockClockPubSub())
    return initResource(clock, empty_season())
}
function empty_winter() {
    const clock = mockClock(winter_now(), mockClockPubSub())
    return initResource(clock, empty_season())
}
function empty_last_winter() {
    const clock = mockClock(last_winter_now(), mockClockPubSub())
    return initResource(clock, empty_season())
}

function initResource(
    clock: Clock,
    seasonRepository: SeasonRepository,
): Readonly<{ season: LoadSeasonAction }> {
    return {
        season: initLoadSeasonAction({
            defaultSeason: currentSeason(clock),
            availableSeasons: standard_availableSeasons(),
            seasonRepository,
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
    return convertDB(db, seasonRepositoryConverter(standard_availableSeasons()))
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
    return convertDB(db, seasonRepositoryConverter(standard_availableSeasons()))
}
function empty_season(): SeasonRepository {
    return convertDB(initMemoryDB(), seasonRepositoryConverter(standard_availableSeasons()))
}

function standard_availableSeasons(): readonly Season[] {
    return [
        markSeason({ year: 2022, period: "summer" }),
        markSeason({ year: 2021, period: "winter" }),
        markSeason({ year: 2021, period: "summer" }),
    ]
}
