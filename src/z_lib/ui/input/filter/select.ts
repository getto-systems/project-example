import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { PrepareElementState } from "../../prepare/data"
import { SelectResult } from "../../validate/data"

export interface SelectFilterAction<T> {
    readonly input: InputBoardAction<BoardValueStore>
    readonly observe: ObserveBoardFieldAction

    options(): PrepareElementState<readonly T[]>
    clear(): void
}

export type SelectFilterProps<T> = Readonly<{
    initial: SelectResult<T>
    convert: (data: T) => string
}>
export function initSelectFilterAction<T>(props: SelectFilterProps<T>): Readonly<{
    input: SelectFilterAction<T>
    setOptions: { (state: readonly T[]): void }
    pin: () => SelectResult<T>
}> {
    const { input, store, subscriber } = initInputBoardAction()
    let options: PrepareElementState<readonly T[]> = { type: "initial" }
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    if (props.initial.isSelected) {
        store.set(props.convert(props.initial.value))
    }

    subscriber.subscribe(() => {
        observe.check()
    })

    return {
        input: {
            input,
            observe,

            options: () => options,
            clear: () => {
                store.set("")
            },
        },
        setOptions: (newOptions) => {
            options = { type: "loaded", data: newOptions }
        },
        pin: () => {
            observe.pin()
            return find(store.get(), options, props.convert)
        },
    }
}

function find<T>(
    selected: string,
    options: PrepareElementState<readonly T[]>,
    convert: (data: T) => string,
): SelectResult<T> {
    if (options.type === "initial") {
        return { isSelected: false }
    }

    const value = options.data.find((data) => selected === convert(data))
    if (value === undefined) {
        return { isSelected: false }
    }

    return { isSelected: true, value }
}
