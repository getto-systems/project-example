import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/impl"

import { loadSeason } from "../../load_season/method"

import { LoadSeasonInfra } from "../../load_season/infra"

import {
    initialLoadSeasonCoreState,
    LoadSeasonCoreAction,
    LoadSeasonCoreMaterial,
    LoadSeasonCoreState,
} from "./action"

export function initLoadSeasonCoreAction(infra: LoadSeasonInfra): LoadSeasonCoreAction {
    return new Action({
        loadSeason: loadSeason(infra),
    })
}

class Action extends ApplicationAbstractStateAction<LoadSeasonCoreState> {
    initialState = initialLoadSeasonCoreState

    material: LoadSeasonCoreMaterial

    constructor(material: LoadSeasonCoreMaterial) {
        super(() => this.material.loadSeason(this.post))
        this.material = material
    }
}
