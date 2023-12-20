import { test, expect } from "vitest"
import { observeAtom } from "../../../z_vendor/getto-atom/test_helper"
import { markSeason } from "../kernel/test_helper"

import { mockClock, mockClockPubSub } from "../../../common/util/clock/mock"
import { initMemoryDB } from "../../../common/util/repository/detail/memory"

import { seasonLabel } from "../kernel/helper"

import { initLoadSeasonAction, LoadSeasonAction } from "./action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../common/util/clock/infra"

import { seasonRepositoryConverter } from "../kernel/convert"
import { convertDB } from "../../../common/util/repository/detail/convert"

import { Season } from "../kernel/data"

test("load from repository", async () => {
    const { season } = standard()

    const result = observeAtom(season.state)

    await season.state.ignitionState

    expect(result()).toEqual([
        {
            type: "success",
            season: { default: false, season: { year: 2022, period: "summer" } },
        },
    ])
})

test("expired; use default", async () => {
    const { season } = expired()

    const result = observeAtom(season.state)

    await season.state.ignitionState

    expect(result()).toEqual([
        {
            type: "success",
            season: { default: true },
        },
    ])
})

test("not found; use default", async () => {
    const { season } = empty_summer()

    const result = observeAtom(season.state)

    await season.state.ignitionState

    expect(result()).toEqual([
        {
            type: "success",
            season: { default: true },
        },
    ])
})

test("not found; use default; winter", async () => {
    const { season } = empty_winter()

    const result = observeAtom(season.state)

    await season.state.ignitionState

    expect(result()).toEqual([
        {
            type: "success",
            season: { default: true },
        },
    ])
})

test("not found; use default; last winter", async () => {
    const { season } = empty_last_winter()

    const result = observeAtom(season.state)

    await season.state.ignitionState

    expect(result()).toEqual([
        {
            type: "success",
            season: { default: true },
        },
    ])
})

test("season label", () => {
    expect(seasonLabel({ default: true })).toEqual("今シーズン")
    expect(
        seasonLabel({ default: false, season: markSeason({ year: 2021, period: "summer" }) }),
    ).toEqual("2021 夏")
    expect(
        seasonLabel({ default: false, season: markSeason({ year: 2021, period: "winter" }) }),
    ).toEqual("2021 冬")
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
