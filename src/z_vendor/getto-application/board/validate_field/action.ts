import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

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
    return new Action(infra)
}

class Action<T, E>
    extends AbstractStatefulApplicationAction<ValidateBoardFieldState<T, E>>
    implements ValidateBoardFieldAction<T, E>
{
    readonly initialState: ValidateBoardFieldState<T, E> = { type: "initial" }

    infra: ValidateBoardFieldInfra<T, E>

    constructor(infra: ValidateBoardFieldInfra<T, E>) {
        super()
        this.infra = infra
    }

    check(): ValidateBoardFieldResult<T, E> {
        const { convert } = this.infra
        const result = convert()
        this.post({ type: "validated", result })
        return result
    }
    clear(): ValidateBoardFieldState<T, E> {
        return this.post({ type: "initial" })
    }
}
