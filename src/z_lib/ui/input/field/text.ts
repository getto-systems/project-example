import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/validate_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"
import { ValidateTextError } from "../../validate/data"

export interface TextFieldAction<T> {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<T, readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction

    clear(): void
    reset(value: T): void
}

export type TextFieldProps<T> = Readonly<{
    convert: (value: string) => ValidateBoardFieldResult<T, readonly ValidateTextError[]>
}>
export function initTextFieldAction<T extends string>(
    props: TextFieldProps<T>,
): TextFieldAction<T> {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        convert: () => props.convert(store.get()),
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
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
        },
        reset: (value) => {
            store.set(value)
        },
    }
}
