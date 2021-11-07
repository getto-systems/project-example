import { ApplicationStateAction } from "../../action/action"

import { UpdateValidateBoardStateMethod } from "../validate_board/method"

import { ValidateBoardState } from "../validate_board/data"

export interface ValidateBoardAction extends ApplicationStateAction<ValidateBoardActionState> {
    clear(): void
}

export type ValidateBoardMaterial<N extends string> = Readonly<{
    update: UpdateValidateBoardStateMethod<N>
}>

export type ValidateBoardActionState = ValidateBoardState
export const initialValidateBoardState: ValidateBoardActionState = "initial"
