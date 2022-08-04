import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../z_vendor/getto-application/board/editable/action"
import { ConvertBoardResult } from "../../../z_vendor/getto-application/board/kernel/data"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../z_vendor/getto-application/board/observe_board/action"
import { ObserveBoardFieldAction } from "../../../z_vendor/getto-application/board/observe_field/action"
import {
    initValidateBoardAction,
    ValidateBoardAction,
} from "../../../z_vendor/getto-application/board/validate_board/action"
import { ValidateBoardFieldAction } from "../../../z_vendor/getto-application/board/validate_field/action"
import { PrepareElementState } from "../prepare/data"

export interface ModifyFieldAction<T> {
    readonly observe: ObserveBoardFieldAction
    readonly validate: ValidateBoardFieldAction<T, unknown>
    reset(value: T): void
    clear(): void
}

export type ModifyFieldEntry<K extends string, T, R> = [K, ModifyFieldAction<T>, (data: R) => void]

export type ModifyFieldProps<T> = EditableDataProps<T> &
    Readonly<{
        validate: ValidateBoardAction
        observe: ObserveBoardAction
        reset: () => void
    }>
export type EditableDataProps<T> = Readonly<{
    editable: EditableBoardAction
    data: () => PrepareElementState<T>
    handler: ModifyFieldHandler<T>
}>

export interface ModifyFieldHandler<T> {
    focus(data: T): void
    update(data: T): void
    close(): void
}

export function modifyField<K extends string, T, R>(
    key: K,
    input: ModifyFieldAction<T>,
    map: (data: R) => T,
): ModifyFieldEntry<K, T, R> {
    return [key, input, (data: R) => input.reset(map(data))]
}

export function initModifyField<K extends string, T, M>(
    entries: readonly ModifyFieldEntry<K, unknown, T>[],
    convert: () => ConvertBoardResult<M>,
): ModifyFieldProps<T> {
    const fields = entries.map(([field]) => field)

    const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
    const { observe, observeChecker } = initObserveBoardAction({ fields })
    const { editable, data, handler } = initEditableDataHandler<T>()

    entries.forEach(([field, input]) => {
        input.validate.state.subscribe((state) => {
            validateChecker.update(field, state)
        })
        input.observe.state.subscribe((result) => {
            observeChecker.update(field, result.hasChanged)
        })
    })

    const clear = () => {
        entries.forEach(([_field, input, _set]) => {
            input.clear()
        })
        validate.clear()
        observe.clear()
    }
    const resetTo = (data: T) => {
        entries.forEach(([_field, _input, set]) => {
            set(data)
        })
        validate.clear()
        observe.clear()
    }
    const reset = () => {
        const element = data()
        if (element.isLoad) {
            resetTo(element.data)
        } else {
            clear()
        }
    }

    editable.state.subscribe((state) => {
        if (state.isEditable) {
            reset()
        } else {
            clear()
        }
    })

    return {
        observe,
        validate,
        editable,
        data,
        handler,
        reset,
    }
}

export function initEditableDataHandler<T>(): EditableDataProps<T> {
    const editable = initEditableBoardAction()

    let element: PrepareElementState<T> = { isLoad: false }

    const initData = () => {
        element = { isLoad: false }
    }
    const setData = (data: T) => {
        element = { isLoad: true, data }
    }

    return {
        editable,
        data: () => element,
        handler: {
            focus: (data: T) => {
                setData(data)
                editable.close()
            },
            update: (data: T) => {
                setData(data)
            },
            close: () => {
                initData()
                editable.close()
            },
        },
    }
}
