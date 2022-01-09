import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

import { BoardFieldChecker, BoardFieldConverter } from "./infra"

import { ConvertBoardFieldResult, ValidateBoardFieldResult } from "../validate_field/data"

export interface ValidateBoardFieldAction<E>
    extends StatefulApplicationAction<ValidateBoardFieldState<E>> {
    clear(): ValidateBoardFieldState<E>
}

export type ValidateBoardFieldState<E> = ValidateBoardFieldResult<E>

export type ValidateBoardFieldInfra<T, E> = Readonly<{
    converter: BoardFieldConverter<T, E>
}>

export function initValidateBoardFieldAction<T, E>(
    infra: ValidateBoardFieldInfra<T, E>,
): Readonly<{ validate: ValidateBoardFieldAction<E>; checker: BoardFieldChecker<T, E> }> {
    const action = new Action(infra)
    return {
        validate: action,
        checker: action,
    }
}

class Action<T, E>
    extends AbstractStatefulApplicationAction<ValidateBoardFieldState<E>>
    implements ValidateBoardFieldAction<E>, BoardFieldChecker<T, E>
{
    readonly initialState: ValidateBoardFieldState<E> = { valid: true }

    infra: ValidateBoardFieldInfra<T, E>

    constructor(infra: ValidateBoardFieldInfra<T, E>) {
        super()
        this.infra = infra
    }

    check(): ConvertBoardFieldResult<T, E> {
        const { converter } = this.infra
        const result = converter()
        if (result.valid) {
            this.post({ valid: true })
        } else {
            this.post(result)
        }
        return result
    }

    clear(): ValidateBoardFieldState<E> {
        return this.post({ valid: true })
    }
}
