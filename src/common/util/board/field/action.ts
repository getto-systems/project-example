import { Atom } from "../../../../z_vendor/getto-atom/atom"
import {
    SingleBoard,
    initSingleBoard,
    initMultipleBoard,
    MultipleBoard,
    BoardInitializer,
    composeBoardInitializer,
} from "../input/action"
import {
    composeObserveBoardAtom,
    initObserveBoardValueAtom,
    initObserveMultipleBoardValueAtom,
    ObserveBoardState,
} from "../observe/action"
import {
    initValidateTextBoardValueAtom,
    initValidateMultipleBoardValueAtom,
    initValidateSelectBoardValueAtom,
    initValidateVectorBoardValueAtom,
    initValidateBoardAtom,
    ValidateBoardState,
} from "../validate/action"
import { EditableBoardAction, initEditableBoardAction } from "../editable/action"
import { LoadState } from "../../load/data"

import { SelectBoardValueError, ValidateBoardValue } from "../validate/data"

export type TextFieldBoard<T, E> = SingleBoard &
    Readonly<{
        validate: Atom<ValidateBoardValue<T, E>>
        observe: Atom<ObserveBoardState>
    }>

export function initTextFieldBoard<T extends { toString(): string }, E>(
    infra: Readonly<{
        convert: (value: string) => ValidateBoardValue<T, E>
    }>,
): [TextFieldBoard<T, E>, BoardInitializer<T>] {
    const [board, initializer] = initSingleBoard()
    return [
        {
            ...board,
            validate: initValidateTextBoardValueAtom(board, infra),
            observe: initObserveBoardValueAtom(board),
        },
        initializer,
    ]
}

export type VectorFieldBoard<T, E> = MultipleBoard &
    Readonly<{
        validate: Atom<ValidateBoardValue<readonly T[], readonly E[]>>
        observe: Atom<ObserveBoardState>
    }>

export function initVectorFieldBoard<T extends { toString(): string }, E>(
    infra: Readonly<{
        convert: (value: string) => ValidateBoardValue<T, E>
    }>,
): [VectorFieldBoard<T, E>, BoardInitializer<readonly T[]>] {
    const [board, initializer] = initMultipleBoard()
    return [
        {
            ...board,
            validate: initValidateVectorBoardValueAtom(board, infra),
            observe: initObserveMultipleBoardValueAtom(board),
        },
        initializer,
    ]
}

export type SelectFieldBoard<T> = SingleBoard &
    Readonly<{
        options: Atom<LoadState<readonly T[]>>
        validate: Atom<ValidateBoardValue<T, SelectBoardValueError>>
        observe: Atom<ObserveBoardState>
    }>

export function initSelectFieldBoard<T extends { toString(): string }>(
    options: Atom<LoadState<readonly T[]>>,
    infra: Readonly<{
        convert: (data: T) => string
    }>,
): [SelectFieldBoard<T>, BoardInitializer<T>] {
    const [board, initializer] = initSingleBoard()
    return [
        {
            ...board,
            options,
            validate: initValidateSelectBoardValueAtom(board, options, infra),
            observe: initObserveBoardValueAtom(board),
        },
        initializer,
    ]
}

export type MultipleFieldBoard<T> = MultipleBoard &
    Readonly<{
        options: Atom<LoadState<readonly T[]>>
        validate: Atom<ValidateBoardValue<readonly T[], SelectBoardValueError>>
        observe: Atom<ObserveBoardState>
    }>

export function initMultipleFieldBoard<T extends { toString(): string }>(
    options: Atom<LoadState<readonly T[]>>,
    infra: Readonly<{
        convert: (data: T) => string
    }>,
): [MultipleFieldBoard<T>, BoardInitializer<readonly T[]>] {
    const [board, initializer] = initMultipleBoard()
    return [
        {
            ...board,
            options,
            validate: initValidateMultipleBoardValueAtom(board, options, infra),
            observe: initObserveMultipleBoardValueAtom(board),
        },
        initializer,
    ]
}

export function composeRegisterFieldBoard(
    fields: ReadonlyArray<
        [
            {
                validate: Atom<ValidateBoardValue<unknown, unknown>>
                observe: Atom<ObserveBoardState>
            },
            BoardInitializer<unknown>,
        ]
    >,
): Readonly<{
    validate: Atom<ValidateBoardState>
    observe: Atom<ObserveBoardState>
    reset: () => void
}> {
    const validate = initValidateBoardAtom(fields.map(([field, _initializer]) => field.validate))
    const observe = composeObserveBoardAtom(fields.map(([field, _initializer]) => field.observe))
    const { reset } = composeBoardInitializer(fields.map(([_field, initializer]) => initializer))
    return {
        validate,
        observe,
        reset,
    }
}

export type ModifyBoardField<M, T, E> = [
    Readonly<
        [
            {
                validate: Atom<ValidateBoardValue<T, E>>
                observe: Atom<ObserveBoardState>
            },
            BoardInitializer<T>,
        ]
    >,
    (data: M) => T,
]

export function composeModifyFieldBoard<M>(
    data: Atom<LoadState<M>>,
    fields: readonly ModifyBoardField<M, unknown, unknown>[],
): Readonly<{
    editable: EditableBoardAction
    validate: Atom<ValidateBoardState>
    observe: Atom<ObserveBoardState>
    reset: () => void
}> {
    const validate = initValidateBoardAtom(
        fields.map(([[field, _initializer], _mapper]) => field.validate),
    )
    const observe = composeObserveBoardAtom(
        fields.map(([[field, _initializer], _mapper]) => field.observe),
    )
    const { reset } = composeBoardInitializer(
        fields.map(([[_field, initializer], _mapper]) => initializer),
    )

    const editable = initEditableBoardAction()
    editable.state.subscribe((state) => {
        if (state.isEditable) {
            reset()
        }
    })

    data.subscribe((data) => {
        editable.close()

        if (data.isLoad) {
            fields.forEach(([[_field, initializer], mapper]) => {
                initializer.init(mapper(data.data))
            })
        }
    })

    return {
        editable,
        validate,
        observe,
        reset,
    }
}
