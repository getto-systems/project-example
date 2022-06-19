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

export type SelectFilterProps<T, V> = Readonly<{
    initial: SelectResult<V>
    map: (data: T) => V
    convert: (data: V) => string
}>
export function initSelectFilterAction<T, V>(
    props: SelectFilterProps<T, V>,
): Readonly<{
    input: SelectFilterAction<T>
    setOptions: { (state: readonly T[]): void }
    pin: () => SelectResult<V>
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
                observe.check()
            },
        },
        setOptions: (newOptions) => {
            options = { type: "loaded", data: newOptions }
        },
        pin: () => {
            observe.pin()
            return find(store.get(), options, props.convert, props.map)
        },
    }
}

function find<T, V>(
    selected: string,
    options: PrepareElementState<readonly T[]>,
    convert: (data: V) => string,
    map: (data: T) => V,
): SelectResult<V> {
    if (options.type === "initial") {
        return { isSelected: false }
    }

    const value = options.data.find((data) => selected === convert(map(data)))
    if (value === undefined) {
        return { isSelected: false }
    }

    return { isSelected: true, value: map(value) }
}
