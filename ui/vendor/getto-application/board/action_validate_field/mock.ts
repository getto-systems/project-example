import { ApplicationAbstractStateAction } from "../../action/init"

import { ValidateBoardFieldAction, ValidateBoardFieldState } from "./action"

import { ConvertBoardFieldResult } from "../validate_field/data"

export function mockValidateBoardFieldAction<N extends string, T, E>(
    name: N,
    value: ConvertBoardFieldResult<T, E>,
): ValidateBoardFieldAction<E> {
    return new Mock(name, value)
}

class Mock<T, E>
    extends ApplicationAbstractStateAction<ValidateBoardFieldState<E>>
    implements ValidateBoardFieldAction<E>
{
    readonly initialState: ValidateBoardFieldState<E> = { valid: true }

    readonly name: string
    value: ConvertBoardFieldResult<T, E>

    constructor(name: string, value: ConvertBoardFieldResult<T, E>) {
        super()
        this.name = name
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
