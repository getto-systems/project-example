import { LoadSeasonResource } from "../action_load/resource"

import { LoadSeasonState } from "../action_load/action"
import { FocusSeasonAction, FocusSeasonState } from "./action"

export type FocusSeasonResource = LoadSeasonResource & Readonly<{ focusSeason: FocusSeasonAction }>

export type FocusSeasonResourceState = Readonly<{
    load: LoadSeasonState
    state: FocusSeasonState
}>
