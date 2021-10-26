import { ApplicationAbstractStateAction } from "../../action/init"

import { checkBoardField } from "../observe_field/method"

import {
    ObserveBoardFieldAction,
    ObserveBoardFieldState,
    ObserveBoardFieldMaterial,
    initialObserveBoardFieldState,
} from "./action"

import { ObserveBoardFieldInfra } from "../observe_field/infra"

export function initObserveBoardFieldAction(
    infra: ObserveBoardFieldInfra,
): ObserveBoardFieldAction {
    return new Action({
        check: checkBoardField(infra),
    })
}

class Action
    extends ApplicationAbstractStateAction<ObserveBoardFieldState>
    implements ObserveBoardFieldAction
{
    readonly initialState = initialObserveBoardFieldState

    material: ObserveBoardFieldMaterial

    constructor(material: ObserveBoardFieldMaterial) {
        super()
        this.material = material
    }

    check(): void {
        this.material.check(this.post)
    }
}
