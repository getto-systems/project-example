import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { LoadSeasonMethod } from "../load_season/method"

import { LoadSeasonEvent } from "../load_season/event"

export type LoadSeasonAction = ApplicationStateAction<LoadSeasonState>

export type LoadSeasonMaterial = Readonly<{
    loadSeason: LoadSeasonMethod
}>

export type LoadSeasonState = Readonly<{ type: "initial-season" }> | LoadSeasonEvent

export const initialLoadSeasonState: LoadSeasonState = { type: "initial-season" }
