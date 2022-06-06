import {
    initMultipleInputBoardAction,
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

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { PrepareElementState } from "../../prepare/data"
import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"

export interface MultipleFieldAction<T> {
    readonly input: InputBoardAction<MultipleBoardValueStore>
    readonly validate: ValidateBoardFieldAction<readonly T[], never>
    readonly observe: ObserveBoardFieldAction

    options(): PrepareElementState<readonly T[]>
    clear(): void
    reset(data: readonly T[]): void
}

export type MultipleFieldProps<T> = Readonly<{
    convert: (data: T) => string
}>
export function initMultipleFieldAction<T>(props: MultipleFieldProps<T>): Readonly<{
    input: MultipleFieldAction<T>
    setOptions: { (state: readonly T[]): void }
}> {
    const { input, store, subscriber } = initMultipleInputBoardAction()
    let options: PrepareElementState<readonly T[]> = { type: "initial" }
    const validate = initValidateBoardFieldAction({
        convert: (): ValidateBoardFieldResult<readonly T[], never> => ({
            valid: true,
            value: convert(store.get(), options, props.convert),
        }),
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
                store.set([])
            },
            reset: (data) => {
                store.set(data.map((value) => props.convert(value)))
            },
        },
        setOptions: (newOptions) => {
            options = { type: "loaded", data: newOptions }
        },
    }
}

function convert<T>(
    selected: readonly string[],
    options: PrepareElementState<readonly T[]>,
    convert: (data: T) => string,
): readonly T[] {
    if (options.type === "initial") {
        return []
    }

    return options.data.filter((data) => selected.some((value) => value === convert(data)))
}
