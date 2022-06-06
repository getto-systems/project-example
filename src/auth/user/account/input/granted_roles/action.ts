import {
    initMultipleInputBoardAction,
    InputBoardAction,
} from "../../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../../z_vendor/getto-application/board/observe_field/init/observer"
import { isSameMultipleBoardValue } from "../../../../../z_vendor/getto-application/board/observe_field/helper"
import { toGrantedRoles } from "./convert"

import { MultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/infra"

import { AuthRole } from "../../../kernel/data"

export interface FilterGrantedRolesAction {
    readonly input: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    clear(): void
}

export function initFilterGrantedRolesAction(initial: readonly AuthRole[]): Readonly<{
    input: FilterGrantedRolesAction
    pin: { (): readonly AuthRole[] }
}> {
    const { input, store, subscriber } = initMultipleInputBoardAction()
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
            isSame: isSameMultipleBoardValue,
        }),
    })

    store.set(initial)

    subscriber.subscribe(() => {
        observe.check()
    })

    return {
        input: {
            input,
            observe,

            clear: () => {
                store.set([])
                observe.check()
            },
        },
        pin: () => {
            observe.pin()
            return toGrantedRoles(store.get())
        },
    }
}
