import { ApplicationAbstractStateAction } from "../../action/init"

import { initObserveBoardStack } from "../observe_board/init/stack"

import { updateObserveBoardState } from "../observe_board/method"

import { ObserveBoardChecker, ObserveBoardStore } from "../observe_board/infra"

import {
    initialObserveBoardState,
    ObserveBoardAction,
    ObserveBoardMaterial,
    ObserveBoardActionState,
} from "./action"

export type ObserveBoardActionParams<N extends string> = Readonly<{
    fields: readonly N[]
}>
export function initObserveBoardAction<N extends string>({
    fields,
}: ObserveBoardActionParams<N>): Readonly<{
    observe: ObserveBoardAction
    checker: ObserveBoardChecker<N>
}> {
    const store: ObserveBoardStore = {
        stack: initObserveBoardStack(),
    }
    const action = new Action({
        update: updateObserveBoardState(fields, store),
    })
    return {
        observe: action,
        checker: action,
    }
}

class Action<N extends string>
    extends ApplicationAbstractStateAction<ObserveBoardActionState>
    implements ObserveBoardAction, ObserveBoardChecker<N>
{
    readonly initialState: ObserveBoardActionState = initialObserveBoardState

    material: ObserveBoardMaterial<N>

    constructor(material: ObserveBoardMaterial<N>) {
        super()
        this.material = material
    }

    update(name: N, hasChanged: boolean): void {
        this.material.update(name, hasChanged, this.post)
    }
}
