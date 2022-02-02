import { StatefulApplicationAction } from "../../../../ui/vendor/getto-application/action/action"

import { InputSeasonAction } from "../input/action_input/action"

import { FocusSeasonMethod } from "../focus/method"

import { FocusSeasonEvent } from "../focus/event"

export interface FocusSeasonAction extends StatefulApplicationAction<FocusSeasonState> {
    readonly season: InputSeasonAction

    open(): Promise<FocusSeasonState>
    focus(): Promise<FocusSeasonState>
}

export type FocusSeasonMaterial = Readonly<{
    focusSeason: FocusSeasonMethod
}>

export type FocusSeasonState =
    | Readonly<{ type: "initial-focus" }>
    | Readonly<{ type: "edit-season" }>
    | FocusSeasonEvent

export const initialFocusSeasonState: FocusSeasonState = { type: "initial-focus" }
