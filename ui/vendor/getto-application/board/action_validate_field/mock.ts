import { ApplicationAbstractStateAction } from "../../action/init"

import { ValidateBoardFieldAction, ValidateBoardFieldState } from "./action"

import { ConvertBoardFieldResult } from "../validate_field/data"

export function mockValidateBoardFieldAction<T, E>(
    value: ConvertBoardFieldResult<T, E>,
): ValidateBoardFieldAction<E> {
    return new Mock(value)
}

class Mock<T, E>
    extends ApplicationAbstractStateAction<ValidateBoardFieldState<E>>
    implements ValidateBoardFieldAction<E>
{
    readonly initialState: ValidateBoardFieldState<E> = { valid: true }

    value: ConvertBoardFieldResult<T, E>

    constructor(value: ConvertBoardFieldResult<T, E>) {
        super()
        this.value = value
    }

    get(): ConvertBoardFieldResult<T, E> {
        return this.value
    }
    async check(): Promise<ValidateBoardFieldState<E>> {
        return this.initialState
    }
    clear(): void {
        return
    }
}
