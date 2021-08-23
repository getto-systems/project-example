import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { loadSeason } from "../load_season/method"

import { LoadSeasonInfra } from "../load_season/infra"

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

class Action extends ApplicationAbstractStateAction<LoadSeasonState> {
    readonly initialState = initialLoadSeasonState

    material: LoadSeasonMaterial

    constructor(material: LoadSeasonMaterial) {
        super(() => this.material.loadSeason(this.post))
        this.material = material
    }
}
