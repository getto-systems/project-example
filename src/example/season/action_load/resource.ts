import { LoadSeasonAction, LoadSeasonState } from "./action"

export type LoadSeasonResource = Readonly<{
    season: LoadSeasonAction
}>

export type LoadSeasonResourceState = Readonly<{
    state: LoadSeasonState
}>
