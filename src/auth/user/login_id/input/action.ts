import { loginIdBoardConverter } from "./convert"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

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

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { SingleValueFilter } from "../../../../z_lib/ui/search/kernel/data"
import { LoginId } from "../kernel/data"
import { ValidateTextError } from "../../../../z_lib/ui/validate/data"

export interface InputLoginIdAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<LoginId, readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export function initInputLoginIdAction(): InputLoginIdAction {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        convert: () => loginIdBoardConverter(store.get()),
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({ current: () => store.get() }),
    })

    subscriber.subscribe(() => {
        validate.check()
        observe.check()
    })

    return {
        input,
        validate,
        observe,
        clear: () => {
            store.set("")
            validate.clear()
            observe.pin()
        },
    }
}

export interface FilterLoginIdAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly observe: ObserveBoardFieldAction
    clear(): void
}

export function initFilterLoginIdAction(initial: SingleValueFilter): Readonly<{
    input: FilterLoginIdAction
    pin: { (): SingleValueFilter }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    if (initial.filter) {
        store.set(initial.value)
    }

    const value = (): SingleValueFilter => {
        const value = store.get()
        if (value === "") {
            return { filter: false }
        }
        return { filter: true, value }
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
        },
        pin: () => {
            observe.pin()
            return value()
        },
    }
}
