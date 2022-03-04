import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

import { ObserveBoardFieldResult } from "../observe_field/data"
import { BoardFieldObserver } from "./infra"

export interface ObserveBoardFieldAction extends StatefulApplicationAction<ObserveBoardFieldState> {
    check(): ObserveBoardFieldResult
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

    check(): ObserveBoardFieldResult {
        const { observer } = this.infra
        return this.post({ hasChanged: observer.hasChanged() })
    }
}
