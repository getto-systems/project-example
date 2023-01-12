import {
    initMultipleInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"
import { isSameMultipleBoardValue } from "../../../../z_vendor/getto-application/board/observe_field/helper"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { PrepareElementState } from "../../prepare/data"

export interface MultipleFilterAction {
    readonly input: InputBoardAction<MultipleBoardValueStore>
    readonly observe: ObserveBoardFieldAction

    clear(): void
}

export type MultipleFilterProps<T> = Readonly<{
    initial: readonly T[]
    convert: (data: T) => string
}>
export function initMultipleFilterAction<T>(props: MultipleFilterProps<T>): Readonly<{
    input: MultipleFilterAction
    setOptions: { (state: readonly T[]): void }
    pin: () => readonly T[]
}> {
    const { input, store, subscriber } = initMultipleInputBoardAction()
    let options: PrepareElementState<readonly T[]> = { isLoad: false }
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
            isSame: isSameMultipleBoardValue,
        }),
    })

    store.set(props.initial.map((value) => props.convert(value)))

    subscriber.subscribe(() => {
        observe.check()
    })

    return {
        input: {
            input,
            observe,

            clear: () => {
                store.set([])
            },
        },
        setOptions: (newOptions) => {
            options = { isLoad: true, data: newOptions }
        },
        pin: () => {
            observe.pin()
            return filter(store.get(), options, props.convert)
        },
    }
}

function filter<T>(
    selected: readonly string[],
    options: PrepareElementState<readonly T[]>,
    convert: (data: T) => string,
): readonly T[] {
    if (!options.isLoad) {
        return []
    }

    return options.data.filter((option) => selected.some((value) => value === convert(option)))
}
