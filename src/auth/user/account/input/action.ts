import { ApplicationAction } from "../../../../z_vendor/getto-application/action/action"
import {
    initMultipleInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"
import { toGrantedRoles } from "./convert"
import { toBoardValue } from "../../../../z_vendor/getto-application/board/kernel/convert"
import { isSameMultipleBoardValue } from "../../../../z_vendor/getto-application/board/observe_field/helper"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { AuthRole } from "../../kernel/data"

export interface InputGrantedRolesAction extends ApplicationAction {
    readonly grantedRoles: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    reset(grantedRoles: readonly AuthRole[]): void
}

export function initInputGrantedRolesAction(): Readonly<{
    input: InputGrantedRolesAction
    convert: { (): readonly AuthRole[] }
}> {
    const input = new InputAction()
    return {
        input,
        convert: () => toGrantedRoles(input.store.grantedRoles.get()),
    }
}

class InputAction implements InputGrantedRolesAction {
    readonly grantedRoles: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    readonly store: Readonly<{
        grantedRoles: MultipleBoardValueStore
    }>

    terminate: { (): void }

    constructor() {
        const grantedRoles = initMultipleInputBoardAction()
        const observe = initObserveBoardFieldAction({
            observer: initBoardFieldObserver({
                current: () => grantedRoles.store.get(),
                isSame: isSameMultipleBoardValue,
            }),
        })

        this.grantedRoles = grantedRoles.input
        this.observe = observe

        this.store = {
            grantedRoles: grantedRoles.store,
        }

        grantedRoles.subscriber.subscribe(() => {
            observe.check()
        })

        this.terminate = () => {
            grantedRoles.subscriber.terminate()
            observe.terminate()
        }
    }

    reset(grantedRoles: readonly AuthRole[]): void {
        this.store.grantedRoles.set(grantedRoles.map(toBoardValue))
    }
}

export interface SearchGrantedRolesAction extends ApplicationAction {
    readonly grantedRoles: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    clear(): void
}

export function initSearchGrantedRolesAction(initial: readonly AuthRole[]): Readonly<{
    input: SearchGrantedRolesAction
    pin: { (): readonly AuthRole[] }
}> {
    const input = new SearchAction()
    input.store.grantedRoles.set(initial.map(toBoardValue))
    return {
        input,
        pin: () => {
            input.observe.pin()
            return toGrantedRoles(input.store.grantedRoles.get())
        },
    }
}

class SearchAction implements SearchGrantedRolesAction {
    readonly grantedRoles: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    readonly store: Readonly<{
        grantedRoles: MultipleBoardValueStore
    }>

    terminate: { (): void }

    constructor() {
        const grantedRoles = initMultipleInputBoardAction()
        const observe = initObserveBoardFieldAction({
            observer: initBoardFieldObserver({
                current: () => grantedRoles.store.get(),
                isSame: isSameMultipleBoardValue,
            }),
        })

        this.grantedRoles = grantedRoles.input
        this.observe = observe

        this.store = {
            grantedRoles: grantedRoles.store,
        }

        grantedRoles.subscriber.subscribe(() => {
            observe.check()
        })

        this.terminate = () => {
            grantedRoles.subscriber.terminate()
            observe.terminate()
        }
    }

    clear(): void {
        this.store.grantedRoles.set([])
        this.observe.check()
    }
}
