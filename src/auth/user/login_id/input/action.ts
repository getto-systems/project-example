import { loginIdBoardConverter } from "./convert"

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
} from "../../../../z_vendor/getto-application/board/validate_field/action"

import { BoardFieldChecker } from "../../../../z_vendor/getto-application/board/validate_field/infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { SingleValueFilter } from "../../../../z_lib/ui/search/kernel/data"
import { LoginId } from "../kernel/data"
import { ValidateTextError } from "../../../../z_lib/ui/validate/data"

export interface InputLoginIdAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export function initInputLoginIdAction(): Readonly<{
    input: InputLoginIdAction
    checker: BoardFieldChecker<LoginId, readonly ValidateTextError[]>
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const { validate, checker } = initValidateBoardFieldAction({
        converter: () => loginIdBoardConverter(store.get()),
    })

    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({ current: () => store.get() }),
    })

    subscriber.subscribe(() => {
        checker.check()
        observe.check()
    })

    return {
        input: {
            input,
            validate,
            observe,
            clear: () => {
                store.set("")
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

export interface SearchLoginIdAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export function initSearchLoginIdAction(initial: SingleValueFilter): Readonly<{
    input: SearchLoginIdAction
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
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({ current: value }),
    })

    subscriber.subscribe(() => observe.check())

    return {
        input: {
            input,
            observe,
            clear: () => {
                store.set("")
                observe.check()
            },
            terminate: () => {
                subscriber.terminate()
                observe.terminate()
            },
        },
        pin: () => {
            observe.pin()
            return value()
        },
    }
}
