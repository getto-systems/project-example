import { ApplicationStateAction } from "../../action/action"

import { ConvertBoardFieldMethod } from "../validate_field/method"

import { ValidateBoardFieldResult } from "../validate_field/data"

export interface ValidateBoardFieldAction<E>
    extends ApplicationStateAction<ValidateBoardFieldState<E>> {
    clear(): void
}

export type ValidateBoardFieldMaterial<T, E> = Readonly<{
    convert: ConvertBoardFieldMethod<T, E>
}>

export type ValidateBoardFieldState<E> = ValidateBoardFieldResult<E>
