import { ApplicationStateAction } from "../../../action/action"

import { ConvertBoardFieldMethod } from "../../validate_field/method"

import { ConvertBoardFieldResult, ValidateBoardFieldResult } from "../../validate_field/data"

export interface ValidateBoardFieldAction<T, E>
    extends ApplicationStateAction<ValidateBoardFieldState<E>> {
    get(): ConvertBoardFieldResult<T, E>
    check(): Promise<ValidateBoardFieldState<E>>
}

export type ValidateBoardFieldMaterial<T, E> = Readonly<{
    convert: ConvertBoardFieldMethod<T, E>
}>

export type ValidateBoardFieldState<E> = ValidateBoardFieldResult<E>

export interface ValidateBoardFieldStateHandler<E> {
    (state: ValidateBoardFieldState<E>): void
}
