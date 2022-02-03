import { StatefulApplicationAction } from "../../../../ui/vendor/getto-application/action/action"

import { LoadSeasonMethod } from "../load/method"

import { LoadSeasonEvent } from "../load/event"

export type LoadSeasonAction = StatefulApplicationAction<LoadSeasonState>

export type LoadSeasonMaterial = Readonly<{
    loadSeason: LoadSeasonMethod
}>

export type LoadSeasonState = Readonly<{ type: "initial-season" }> | LoadSeasonEvent

export const initialLoadSeasonState: LoadSeasonState = { type: "initial-season" }
