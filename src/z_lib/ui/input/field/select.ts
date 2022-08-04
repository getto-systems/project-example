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

import { PrepareElementState } from "../../prepare/data"
import { ValidateSelectError } from "../../validate/data"
import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"

export interface SelectFieldAction<T> {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<T, ValidateSelectError>
    readonly observe: ObserveBoardFieldAction

    options(): PrepareElementState<readonly T[]>
    clear(): void
    reset(data: T): void
}

export type SelectFieldProps<T> = Readonly<{
    convert: (data: T) => string
}>
export function initSelectFieldAction<T>(props: SelectFieldProps<T>): Readonly<{
    input: SelectFieldAction<T>
    setOptions: { (state: readonly T[]): void }
}> {
    const { input, store, subscriber } = initInputBoardAction()
    let options: PrepareElementState<readonly T[]> = { isLoad: false }
    const validate = initValidateBoardFieldAction({
        convert: () => convert(store.get(), options, props.convert),
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
        input: {
            input,
            validate,
            observe,

            options: () => options,
            clear: () => {
                store.set("")
            },
            reset: (data) => {
                store.set(props.convert(data))
            },
        },
        setOptions: (newOptions) => {
            options = { isLoad: true, data: newOptions }
        },
    }
}

function convert<T>(
    selected: string,
    options: PrepareElementState<readonly T[]>,
    convert: (data: T) => string,
): ValidateBoardFieldResult<T, ValidateSelectError> {
    if (!options.isLoad) {
        return { valid: false, err: { type: "not-selected" } }
    }

    const value = options.data.find((data) => selected === convert(data))
    if (value === undefined) {
        return { valid: false, err: { type: "not-selected" } }
    }

    return { valid: true, value }
}
