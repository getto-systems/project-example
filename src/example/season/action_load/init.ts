import { AbstractStatefulApplicationAction } from "../../../../ui/vendor/getto-application/action/action"

import { loadSeason } from "../load/method"

import { LoadSeasonInfra } from "../load/infra"

import {
    initialLoadSeasonState,
    LoadSeasonAction,
    LoadSeasonMaterial,
    LoadSeasonState,
} from "./action"

export function initLoadSeasonAction(infra: LoadSeasonInfra): LoadSeasonAction {
    return new Action({
        loadSeason: loadSeason(infra),
    })
}

class Action extends AbstractStatefulApplicationAction<LoadSeasonState> {
    readonly initialState = initialLoadSeasonState

    material: LoadSeasonMaterial

    constructor(material: LoadSeasonMaterial) {
        super({ ignite: () => this.material.loadSeason(this.post) })
        this.material = material
    }
}
