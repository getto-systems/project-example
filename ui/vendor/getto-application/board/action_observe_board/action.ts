import { ApplicationStateAction } from "../../action/action"

import { UpdateObserveBoardStateMethod } from "../observe_board/method"
import { ObserveBoardFieldResult } from "../observe_field/data"

export type ObserveBoardAction = ApplicationStateAction<ObserveBoardActionState>

export type ObserveBoardMaterial<N extends string> = Readonly<{
    update: UpdateObserveBoardStateMethod<N>
}>

export type ObserveBoardActionState = ObserveBoardFieldResult
export const initialObserveBoardState: ObserveBoardActionState = { hasChanged: false }
