import {
    initSingleBoardStoreConnector,
    initFileStoreConnector,
    initMultipleBoardStoreConnector,
} from "./detail/connector"

import { Atom, initAtom } from "../../../../z_vendor/getto-atom/atom"

import {
    SingleBoardStore,
    BoardStoreConnector,
    FileBoardStore,
    MultipleBoardStore,
    ReadonlyBoardStore,
    SelectFileResult,
    WritableBoardStore,
} from "./infra"

export type SingleInputBoardAction = InputBoardAction<SingleBoardStore>
export type MultipleInputBoardAction = InputBoardAction<MultipleBoardStore>
export type FileInputBoardAction = InputBoardAction<FileBoardStore>

export interface InputBoardAction<S> {
    // 例外的に infra をそのまま公開する
    // input 要素を infra に変換するアダプタ
    readonly connector: BoardStoreConnector<S>

    onInput(): void
}

export interface BoardInitializer<T> {
    init(value: T): void
    reset(): void
    pin(): void
}

export type SingleBoard = Readonly<{
    readonly input: SingleInputBoardAction
    readonly value: Atom<string>
    readonly initial: Atom<string>
}>

export function initSingleBoard<T extends { toString(): string }>(): [
    SingleBoard,
    BoardInitializer<T>,
] {
    return initWritableBoard({
        initialState: "",
        initConnector: initSingleBoardStoreConnector,
        toValue: (model: T) => model.toString(),
        passthrough: (value: string) => value,
    })
}

export type MultipleBoard = Readonly<{
    input: MultipleInputBoardAction
    value: Atom<readonly string[]>
    initial: Atom<readonly string[]>
}>

export function initMultipleBoard<T extends { toString(): string }>(): [
    MultipleBoard,
    BoardInitializer<readonly T[]>,
] {
    return initWritableBoard({
        initialState: [],
        initConnector: initMultipleBoardStoreConnector,
        toValue: (model: readonly T[]) => model.map((value) => value.toString()),
        passthrough: (value: readonly string[]) => value,
    })
}

export type FileBoard = Readonly<{
    input: FileInputBoardAction
    value: Atom<SelectFileResult>
}>

export function initFileBoard(): FileBoard {
    return initReadonlyBoard({
        initialState: { found: false },
        initConnector: initFileStoreConnector,
        passthrough: (value: SelectFileResult) => value,
    })
}

type WritableBoard<V, S extends WritableBoardStore<V>> = Readonly<{
    input: InputBoardAction<S>
    value: Atom<ReturnType<S["get"]>>
    initial: Atom<ReturnType<S["get"]>>
}>

function initWritableBoard<T, V, S extends WritableBoardStore<V>>({
    initialState,
    initConnector,
    toValue,
    passthrough,
}: Readonly<{
    initialState: ReturnType<S["get"]>
    initConnector: () => Readonly<{ connector: BoardStoreConnector<S>; store: S }>
    toValue: (model: T) => ReturnType<S["get"]>
    passthrough: (value: V) => ReturnType<S["get"]>
}>): [WritableBoard<V, S>, BoardInitializer<T>] {
    const { connector, store } = initConnector()
    const value = initAtom({ initialState })
    const initial = initAtom({ initialState })

    return [
        {
            input: {
                connector,
                onInput() {
                    value.post(passthrough(store.get()))
                },
            },
            value: value.state,
            initial: initial.state,
        },
        {
            init: (newValue) => reset(toValue(newValue)),
            reset() {
                reset(initial.state.currentState())
            },
            pin(): void {
                initial.post(value.state.currentState())
            },
        },
    ]

    function reset(newValue: ReturnType<S["get"]>) {
        store.set(newValue)
        value.post(newValue)
        initial.post(newValue)
    }
}

type ReadonlyBoard<V, S extends ReadonlyBoardStore<V>> = Readonly<{
    input: InputBoardAction<S>
    value: Atom<ReturnType<S["get"]>>
}>

function initReadonlyBoard<V, S extends ReadonlyBoardStore<V>>({
    initialState,
    initConnector,
    passthrough,
}: Readonly<{
    initialState: ReturnType<S["get"]>
    initConnector: () => Readonly<{ connector: BoardStoreConnector<S>; store: S }>
    passthrough: (value: V) => ReturnType<S["get"]>
}>): ReadonlyBoard<V, S> {
    const { connector, store } = initConnector()
    const value = initAtom({ initialState })

    return {
        input: {
            connector,
            onInput() {
                value.post(passthrough(store.get()))
            },
        },
        value: value.state,
    }
}

export function composeBoardInitializer(
    initializers: readonly Readonly<{
        reset: () => void
        pin: () => void
    }>[],
): Readonly<{
    reset: () => void
    pin: () => void
}> {
    return {
        reset: () => {
            initializers.forEach((filter) => filter.reset())
        },
        pin: () => {
            initializers.forEach((filter) => filter.pin())
        },
    }
}
