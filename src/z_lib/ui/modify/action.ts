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

export interface ModifyFieldAction<T> {
    readonly observe: ObserveBoardFieldAction
    readonly validate: ValidateBoardFieldAction<T, unknown>
    reset(value: T): void
}

export type ModifyFieldEntry<K extends string, T, R> = [K, ModifyFieldAction<T>, (data: R) => void]

export type ModifyFieldProps<R> = Readonly<{
    validate: ValidateBoardAction
    observe: ObserveBoardAction
    reset: (data: R) => void
}>

export function modifyField<K extends string, T, R>(
    key: K,
    input: ModifyFieldAction<T>,
    map: (data: R) => T,
): ModifyFieldEntry<K, T, R> {
    return [key, input, (data: R) => input.reset(map(data))]
}

export function initModifyField<K extends string, R>(
    entries: readonly ModifyFieldEntry<K, unknown, R>[],
    convert: () => ConvertBoardResult<R>,
): ModifyFieldProps<R> {
    const fields = entries.map(([field]) => field)

    const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
    const { observe, observeChecker } = initObserveBoardAction({ fields })

    entries.forEach(([field, input]) => {
        input.validate.subscriber.subscribe((state) => {
            validateChecker.update(field, state)
        })
        input.observe.subscriber.subscribe((result) => {
            observeChecker.update(field, result.hasChanged)
        })
    })

    const reset = (data: R) => {
        entries.forEach(([_field, _input, reset]) => {
            reset(data)
        })
        validate.clear()
        observe.clear()
    }

    return {
        observe,
        validate,
        reset,
    }
}
