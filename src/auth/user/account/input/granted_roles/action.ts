import { ApplicationAction } from "../../../../../z_vendor/getto-application/action/action"
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
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../../../z_vendor/getto-application/board/validate_field/action"
import { ValidateBoardFieldResult } from "../../../../../z_vendor/getto-application/board/validate_field/data"

export interface InputGrantedAuthRolesAction extends ApplicationAction {
    readonly input: InputBoardAction<MultipleBoardValueStore>
    readonly validate: ValidateBoardFieldAction<readonly AuthRole[], false>
    readonly observe: ObserveBoardFieldAction

    reset(grantedRoles: readonly AuthRole[]): void
}

export function initInputGrantedAuthRolesAction(): InputGrantedAuthRolesAction {
    const { input, store, subscriber } = initMultipleInputBoardAction()
    const validate = initValidateBoardFieldAction({
        convert: (): ValidateBoardFieldResult<readonly AuthRole[], false> => ({
            valid: true,
            value: toGrantedRoles(store.get()),
        }),
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
            isSame: isSameMultipleBoardValue,
        }),
    })

    subscriber.subscribe(() => {
        observe.check()
    })

    return {
        input,
        validate,
        observe,

        reset: (grantedRoles) => {
            store.set(grantedRoles)
        },
        terminate: () => {
            subscriber.terminate()
            observe.terminate()
        },
    }
}

export interface SearchGrantedRolesAction extends ApplicationAction {
    readonly input: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    clear(): void
}

export function initSearchGrantedRolesAction(initial: readonly AuthRole[]): Readonly<{
    input: SearchGrantedRolesAction
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

            terminate: () => {
                subscriber.terminate()
                observe.terminate()
            },
        },
        pin: () => {
            observe.pin()
            return toGrantedRoles(store.get())
        },
    }
}
