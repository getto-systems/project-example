import { ApplicationState, initApplicationState } from "../../action/action"

import { ObserveBoardFieldResult } from "../observe_field/data"
import { BoardFieldObserver } from "./infra"

export interface ObserveBoardFieldAction {
    readonly state: ApplicationState<ObserveBoardFieldState>
    pin(): ObserveBoardFieldState
    check(): ObserveBoardFieldState
}

export type ObserveBoardFieldState = ObserveBoardFieldResult
const initialState: ObserveBoardFieldState = { hasChanged: false }

export type ObserveBoardFieldInfra = Readonly<{
    observer: BoardFieldObserver
}>

export function initObserveBoardFieldAction(
    infra: ObserveBoardFieldInfra,
): ObserveBoardFieldAction {
    const { state, post } = initApplicationState({ initialState })
    return { state, pin, check }

    function pin(): ObserveBoardFieldState {
        const { observer } = infra
        observer.pin()
        return check()
    }
    function check(): ObserveBoardFieldState {
        const { observer } = infra
        return post({ hasChanged: observer.hasChanged() })
    }
}
