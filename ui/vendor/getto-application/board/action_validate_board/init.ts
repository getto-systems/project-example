import { ApplicationAbstractStateAction } from "../../action/init"

import { initValidateBoardStack } from "../validate_board/init/stack"

import { updateBoardValidateState } from "../validate_board/method"

import { ValidateBoardStore } from "../validate_board/infra"
import { BoardConverter } from "../kernel/infra"

import { ValidateBoardFieldStateHandler } from "../action_validate_field/action"
import {
    initialValidateBoardState,
    ValidateBoardAction,
    ValidateBoardMaterial,
    ValidateBoardActionState,
} from "./action"

import { ConvertBoardResult } from "../kernel/data"

export type ValidateBoardActionParams<N extends string, T> = Readonly<{
    fields: readonly N[]
    converter: BoardConverter<T>
}>
export function initValidateBoardAction<N extends string, T>({
    fields,
    converter,
}: ValidateBoardActionParams<N, T>): ValidateBoardAction<N, T> {
    const store: ValidateBoardStore = {
        stack: initValidateBoardStack(),
    }
    return new Action(converter, {
        updateValidateState: updateBoardValidateState(fields, store),
    })
}

class Action<N extends string, T>
    extends ApplicationAbstractStateAction<ValidateBoardActionState>
    implements ValidateBoardAction<N, T> {
    readonly initialState: ValidateBoardActionState = initialValidateBoardState

    converter: BoardConverter<T>
    material: ValidateBoardMaterial<N>

    constructor(converter: BoardConverter<T>, material: ValidateBoardMaterial<N>) {
        super()
        this.converter = converter
        this.material = material
    }

    updateValidateState<E>(name: N): ValidateBoardFieldStateHandler<E> {
        return (result) => {
            this.material.updateValidateState(name, result.valid, this.post)
        }
    }

    get(): ConvertBoardResult<T> {
        return this.converter()
    }
    clear(): void {
        this.post(initialValidateBoardState)
    }
}