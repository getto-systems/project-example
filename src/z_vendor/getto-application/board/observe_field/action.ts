import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

import { ObserveBoardFieldResult } from "../observe_field/data"
import { BoardFieldObserver } from "./infra"

export interface ObserveBoardFieldAction extends StatefulApplicationAction<ObserveBoardFieldState> {
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
    return new Action(infra)
}

class Action
    extends AbstractStatefulApplicationAction<ObserveBoardFieldState>
    implements ObserveBoardFieldAction
{
    readonly initialState = initialState

    infra: ObserveBoardFieldInfra

    constructor(infra: ObserveBoardFieldInfra) {
        super()
        this.infra = infra
    }

    pin(): ObserveBoardFieldState {
        const { observer } = this.infra
        observer.pin()
        return this.check()
    }
    check(): ObserveBoardFieldState {
        const { observer } = this.infra
        return this.post({ hasChanged: observer.hasChanged() })
    }
}
