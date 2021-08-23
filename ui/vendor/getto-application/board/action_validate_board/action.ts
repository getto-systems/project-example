import { ApplicationStateAction } from "../../action/action"

import { ValidateBoardFieldStateHandler } from "../action_validate_field/action"

import { UpdateBoardValidateStateMethod } from "../validate_board/method"

import { ValidateBoardState } from "../validate_board/data"
import { ConvertBoardResult } from "../kernel/data"

export interface ValidateBoardAction<N extends string, T>
    extends ApplicationStateAction<ValidateBoardActionState> {
    // TODO これは init で infra として返すべき
    updateValidateState<E>(name: N): ValidateBoardFieldStateHandler<E>
    // TODO これも infra の気がする
    get(): ConvertBoardResult<T>
    clear(): void
}

export type ValidateBoardMaterial<N extends string> = Readonly<{
    updateValidateState: UpdateBoardValidateStateMethod<N>
}>

export type ValidateBoardActionState = ValidateBoardState
export const initialValidateBoardState: ValidateBoardActionState = "initial"
