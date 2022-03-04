import { loginIDBoardConverter } from "./convert"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { ApplicationAction } from "../../../../z_vendor/getto-application/action/action"
import { initObserveBoardFieldAction } from "../../../../z_vendor/getto-application/board/observe_field/action"
import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import { ObserveBoardFieldAction } from "../../../../z_vendor/getto-application/board/observe_field/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../z_vendor/getto-application/board/validate_field/action"

import { BoardFieldChecker } from "../../../../z_vendor/getto-application/board/validate_field/infra"

import { emptyBoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginID, ValidateLoginIDError } from "./data"
import { SingleValueFilter } from "../../../../z_lib/ui/search/kernel/data"

export interface InputLoginIDAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly validate: ValidateLoginIDAction
    clear(): void
}

export type ValidateLoginIDAction = ValidateBoardFieldAction<ValidateLoginIDError>
export type ValidateLoginIDState = ValidateBoardFieldState<ValidateLoginIDError>

export function initInputLoginIDAction(): Readonly<{
    input: InputLoginIDAction
    checker: BoardFieldChecker<LoginID, ValidateLoginIDError>
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const { validate, checker } = initValidateBoardFieldAction({
        converter: () => loginIDBoardConverter(store.get()),
    })

    subscriber.subscribe(() => checker.check())

    return {
        input: {
            input,
            validate,
            clear: () => {
                store.set(emptyBoardValue)
                validate.clear()
            },
            terminate: () => {
                subscriber.terminate()
                validate.terminate()
            },
        },
        checker,
    }
}

export interface SearchLoginIDAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export function initSearchLoginIDAction(initial: SingleValueFilter): Readonly<{
    input: SearchLoginIDAction
    pin: { (): SingleValueFilter }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    if (initial.search) {
        store.set(initial.value)
    }

    const value = (): SingleValueFilter => {
        const value = store.get()
        if (value === "") {
            return { search: false }
        }
        return { search: true, value }
    }
    const observer = initBoardFieldObserver(value)
    const observe = initObserveBoardFieldAction({ observer })

    subscriber.subscribe(() => observe.check())

    return {
        input: {
            input,
            observe,
            clear: () => {
                store.set(emptyBoardValue)
                observe.check()
            },
            terminate: () => {
                subscriber.terminate()
            },
        },
        pin: () => {
            observer.pin()
            return value()
        },
    }
}
