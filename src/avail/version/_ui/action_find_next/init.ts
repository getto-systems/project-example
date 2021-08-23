import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { findNextVersion } from "../find_next/method"

import { FindNextVersionInfra } from "../find_next/infra"

import { FindNextVersionDetecter } from "../find_next/method"

import {
    FindNextVersionMaterial,
    FindNextVersionState,
    initialFindNextVersionState,
    FindNextVersionAction,
} from "./action"

export function initFindNextVersionMaterial(
    infra: FindNextVersionInfra,
    detecter: FindNextVersionDetecter,
): FindNextVersionMaterial {
    return {
        find: findNextVersion(infra)(detecter),
    }
}

export function initFindNextVersionAction(
    material: FindNextVersionMaterial,
): FindNextVersionAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<FindNextVersionState>
    implements FindNextVersionAction {
    readonly initialState = initialFindNextVersionState

    material: FindNextVersionMaterial

    constructor(material: FindNextVersionMaterial) {
        super(() => this.material.find(this.post))
        this.material = material
    }
}
