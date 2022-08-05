import { initApplicationState, StatefulApplicationAction } from "../../action/action"

import { ValidateBoardFieldResult } from "../validate_field/data"

export interface ValidateBoardFieldAction<T, E>
    extends StatefulApplicationAction<ValidateBoardFieldState<T, E>> {
    check(): ValidateBoardFieldResult<T, E>
    clear(): ValidateBoardFieldState<T, E>
}

export type ValidateBoardFieldState<T, E> =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "validated"; result: ValidateBoardFieldResult<T, E> }>

export type ValidateBoardFieldInfra<T, E> = Readonly<{
    convert: { (): ValidateBoardFieldResult<T, E> }
}>

export function initValidateBoardFieldAction<T, E>(
    infra: ValidateBoardFieldInfra<T, E>,
): ValidateBoardFieldAction<T, E> {
    const { state, post } = initApplicationState<ValidateBoardFieldState<T, E>>({
        initialState: { type: "initial" },
    })
    return { state, check, clear }

    function check(): ValidateBoardFieldResult<T, E> {
        const { convert } = infra
        const result = convert()
        post({ type: "validated", result })
        return result
    }
    function clear(): ValidateBoardFieldState<T, E> {
        return post({ type: "initial" })
    }
}
