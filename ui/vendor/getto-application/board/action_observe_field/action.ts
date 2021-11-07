import { ApplicationStateAction } from "../../action/action"

import { CheckBoardFieldMethod } from "../observe_field/method"

import { ObserveBoardFieldResult } from "../observe_field/data"

export interface ObserveBoardFieldAction extends ApplicationStateAction<ObserveBoardFieldState> {
    check(): void
}

export type ObserveBoardFieldMaterial = Readonly<{
    check: CheckBoardFieldMethod
}>

export type ObserveBoardFieldState = ObserveBoardFieldResult
export const initialObserveBoardFieldState: ObserveBoardFieldState = { hasChanged: false }
