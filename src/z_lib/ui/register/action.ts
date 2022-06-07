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

export interface RegisterFieldAction {
    readonly observe: ObserveBoardFieldAction
    readonly validate: ValidateBoardFieldAction<unknown, unknown>
    clear(): void
}

export type RegisterFieldEntry<K extends string> = [K, RegisterFieldAction]

export type RegisterFieldProps = Readonly<{
    validate: ValidateBoardAction
    observe: ObserveBoardAction
    clear: () => void
}>

export function initRegisterField<K extends string>(
    entries: readonly RegisterFieldEntry<K>[],
    convert: () => ConvertBoardResult<unknown>,
): RegisterFieldProps {
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

    const clear = () => {
        entries.forEach(([_field, input]) => {
            input.clear()
        })
        validate.clear()
        observe.clear()
    }

    return {
        observe,
        validate,
        clear,
    }
}
