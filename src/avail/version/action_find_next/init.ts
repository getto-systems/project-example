import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"

import { findNextVersion } from "../find_next/method"

import { FindNextVersionInfra } from "../find_next/infra"

import { FindNextVersionDetecter } from "../find_next/method"

import {
    FindNextVersionMaterial,
    FindNextVersionState,
    initialFindNextVersionState,
    FindNextVersionAction,
} from "./action"

export function initFindNextVersionMaterial(infra: FindNextVersionInfra): FindNextVersionMaterial {
    return {
        find: findNextVersion(infra),
    }
}

export function initFindNextVersionAction(
    material: FindNextVersionMaterial,
    detecter: FindNextVersionDetecter,
): FindNextVersionAction {
    return new Action(material, detecter)
}

class Action
    extends ApplicationAbstractStateAction<FindNextVersionState>
    implements FindNextVersionAction
{
    readonly initialState = initialFindNextVersionState

    constructor(material: FindNextVersionMaterial, detecter: FindNextVersionDetecter) {
        super(() => material.find(detecter(), this.post))
    }
}
