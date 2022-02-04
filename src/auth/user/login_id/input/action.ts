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

import {
    BoardValue,
    emptyBoardValue,
} from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginID, ValidateLoginIDError } from "./data"

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

export function initSearchLoginIDAction(initial: BoardValue): Readonly<{
    input: SearchLoginIDAction
    pin: { (): BoardValue }
    peek: { (): BoardValue }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    store.set(initial)

    const value = () => store.get()
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
        peek: () => {
            return observer.peek()
        },
    }
}
