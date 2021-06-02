import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { LoadSeasonMethod } from "../../load_season/method"

import { LoadSeasonEvent } from "../../load_season/event"

export type LoadSeasonCoreAction = ApplicationStateAction<LoadSeasonCoreState>

export type LoadSeasonCoreMaterial = Readonly<{
    loadSeason: LoadSeasonMethod
}>

export type LoadSeasonCoreState = Readonly<{ type: "initial-season" }> | LoadSeasonEvent

export const initialLoadSeasonCoreState: LoadSeasonCoreState = { type: "initial-season" }
