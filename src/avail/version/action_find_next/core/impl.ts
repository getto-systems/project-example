import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/impl"

import { findNextVersion } from "../../find_next/impl/core"

import { FindNextVersionInfra } from "../../find_next/infra"

import { FindNextVersionLocationDetecter } from "../../find_next/method"

import {
    FindNextVersionMaterial,
    FindNextVersionCoreState,
    initialFindNextVersionCoreState,
    FindNextVersionCoreAction,
} from "./action"

export function initFindNextVersionCoreMaterial(
    infra: FindNextVersionInfra,
    detecter: FindNextVersionLocationDetecter,
): FindNextVersionMaterial {
    return {
        find: findNextVersion(infra)(detecter),
    }
}

export function initFindNextVersionCoreAction(
    material: FindNextVersionMaterial,
): FindNextVersionCoreAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<FindNextVersionCoreState>
    implements FindNextVersionCoreAction {
    readonly initialState = initialFindNextVersionCoreState

    material: FindNextVersionMaterial

    constructor(material: FindNextVersionMaterial) {
        super(() => this.material.find(this.post))
        this.material = material
    }
}
