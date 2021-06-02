import { LoadSeasonCoreAction, LoadSeasonCoreState } from "./core/action"

export type LoadSeasonResource = Readonly<{
    season: LoadSeasonCoreAction
}>

export type LoadSeasonResourceState = Readonly<{
    state: LoadSeasonCoreState
}>
