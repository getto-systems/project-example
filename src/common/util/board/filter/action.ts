import { Atom, combineAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import {
    SingleBoard,
    initSingleBoard,
    initMultipleBoard,
    MultipleBoard,
    BoardInitializer,
    composeBoardInitializer,
} from "../input/action"
import {
    ObserveBoardState,
    composeObserveBoardAtom,
    initObserveBoardValueAtom,
    initObserveMultipleBoardValueAtom,
} from "../observe/action"

import { LoadState } from "../../load/data"
import {
    MultipleFilterBoardFilter,
    MultipleFilterBoardValue,
    SelectFilterBoardFilter,
    SelectFilterBoardValue,
    SingleFilterBoardValue,
} from "./data"

export type SingleFilterBoardInitializer = BoardInitializer<SingleFilterBoardValue>
export type SelectFilterBoardInitializer = BoardInitializer<SelectFilterBoardValue>
export type MultipleFilterBoardInitializer = BoardInitializer<MultipleFilterBoardValue>

export type OffsetFilterBoard = SingleBoard &
    Readonly<{
        observe: Atom<ObserveBoardState>
    }>

export function initOffsetFilterBoard(
    initialOffset: string,
): [OffsetFilterBoard, BoardInitializer<string>] {
    const [board, initializer] = initSingleBoard()
    initializer.init(initialOffset)
    return [
        {
            ...board,
            observe: initObserveBoardValueAtom(board),
        },
        initializer,
    ]
}

export type TextFilterBoard = SingleBoard &
    Readonly<{
        filter: Atom<SingleFilterBoardValue>
        observe: Atom<ObserveBoardState>
    }>

export function initTextFilterBoard(
    initial: SingleFilterBoardValue,
): [TextFilterBoard, SingleFilterBoardInitializer] {
    const [board, boardInitializer] = initSingleBoard()

    const initializer: SingleFilterBoardInitializer = {
        init(value) {
            if (value.length === 0) {
                boardInitializer.init("")
            } else {
                boardInitializer.init(value[0])
            }
        },
        reset() {
            boardInitializer.reset()
        },
        pin() {
            boardInitializer.pin()
        },
    }

    initializer.init(initial)

    return [
        {
            ...board,
            filter: mapAtom(board.value, (value): SingleFilterBoardValue => {
                return value === "" ? [] : [value]
            }),
            observe: initObserveBoardValueAtom(board),
        },
        initializer,
    ]
}

export type SelectFilterBoard<T, F> = SingleBoard &
    Readonly<{
        options: Atom<LoadState<readonly T[]>>
        filter: Atom<[] | [F]>
        observe: Atom<ObserveBoardState>
    }>

export function initSelectFilterBoard<T, F>({
    initial,
    options,
    toFilter,
    toValue,
}: Readonly<{
    initial: SelectFilterBoardValue
    options: Atom<LoadState<readonly T[]>>
    toFilter: (option: T) => F
    toValue: (option: T) => string
}>): [SelectFilterBoard<T, F>, SelectFilterBoardInitializer] {
    const [board, boardInitializer] = initSingleBoard()

    const initializer: SelectFilterBoardInitializer = {
        init(value) {
            if (value.length === 0) {
                boardInitializer.init("")
            } else {
                boardInitializer.init(value[0])
            }
        },
        reset() {
            boardInitializer.reset()
        },
        pin() {
            boardInitializer.pin()
        },
    }

    initializer.init(initial)

    return [
        {
            ...board,
            options,
            filter: combineAtom(
                board.value,
                options,
                (value, options): SelectFilterBoardFilter<F> => {
                    if (!options.isLoad) {
                        return []
                    }
                    for (const option of options.data) {
                        if (value === toValue(option)) {
                            return [toFilter(option)]
                        }
                    }
                    return []
                },
            ),
            observe: initObserveBoardValueAtom(board),
        },
        initializer,
    ]
}

export type MultipleFilterBoard<T, F> = MultipleBoard &
    Readonly<{
        options: Atom<LoadState<readonly T[]>>
        filter: Atom<MultipleFilterBoardFilter<F>>
        observe: Atom<ObserveBoardState>
    }>

export function initMultipleFilterBoard<T, F>({
    initial,
    options,
    toFilter,
    toValue,
}: Readonly<{
    initial: MultipleFilterBoardFilter<string>
    options: Atom<LoadState<readonly T[]>>
    toFilter: (option: T) => F
    toValue: (option: T) => string
}>): [MultipleFilterBoard<T, F>, MultipleFilterBoardInitializer] {
    const [board, initializer] = initMultipleBoard()

    initializer.init(initial)

    return [
        {
            ...board,
            options,
            filter: combineAtom(
                board.value,
                options,
                (value, options): MultipleFilterBoardFilter<F> => {
                    if (!options.isLoad) {
                        return []
                    }

                    const filters: F[] = []
                    for (const option of options.data) {
                        if (value.includes(toValue(option))) {
                            filters.push(toFilter(option))
                        }
                    }
                    return filters
                },
            ),
            observe: initObserveMultipleBoardValueAtom(board),
        },
        initializer,
    ]
}

export function composeSearchFilterBoard(
    offset: { observe: Atom<ObserveBoardState> },
    filters: ReadonlyArray<[{ observe: Atom<ObserveBoardState> }, BoardInitializer<unknown>]>,
): Readonly<{
    observe: Atom<ObserveBoardState>
    reset: () => void
    pin: () => void
}> {
    const observe = composeObserveBoardAtom([
        offset.observe,
        ...filters.map(([filter, _initializer]) => filter.observe),
    ])
    const { reset, pin } = composeBoardInitializer(
        filters.map(([_filter, initializer]) => initializer),
    )
    return {
        observe,
        reset,
        pin,
    }
}

export function mapSelectFilterBoardFilter<A, B>(
    filter: SelectFilterBoardFilter<A>,
    convert: (value: A) => B,
): SelectFilterBoardFilter<B> {
    if (filter.length === 0) {
        return []
    }
    return [convert(filter[0])]
}
