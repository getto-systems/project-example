import { isSameStringArray } from "../kernel/helper"

import { Atom, combineAtom, composeAtom } from "../../../../z_vendor/getto-atom/atom"

export type ObserveBoardState = Readonly<{ hasChanged: boolean }>

export function composeObserveBoardAtom(
    values: readonly Atom<ObserveBoardState>[],
): Atom<ObserveBoardState> {
    return composeAtom(values, (stateArr) => {
        for (const state of stateArr) {
            if (state.hasChanged) {
                return { hasChanged: true }
            }
        }
        return { hasChanged: false }
    })
}

export function initObserveBoardValueAtom(action: {
    value: Atom<string>
    initial: Atom<string>
}): Atom<ObserveBoardState> {
    return combineAtom(action.value, action.initial, (value, initial): ObserveBoardState => {
        return { hasChanged: value !== initial }
    })
}

export function initObserveMultipleBoardValueAtom(action: {
    value: Atom<readonly string[]>
    initial: Atom<readonly string[]>
}): Atom<ObserveBoardState> {
    return combineAtom(action.value, action.initial, (value, initial): ObserveBoardState => {
        return { hasChanged: !isSameStringArray(value, initial) }
    })
}
