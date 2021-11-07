import { ApplicationAbstractStateAction } from "../../action/init"

import { initValidateBoardStack } from "../validate_board/init/stack"

import { updateValidateBoardState } from "../validate_board/method"

import { ValidateBoardChecker, ValidateBoardStore } from "../validate_board/infra"
import { BoardConverter } from "../kernel/infra"

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
}: ValidateBoardActionParams<N, T>): Readonly<{
    validate: ValidateBoardAction
    checker: ValidateBoardChecker<N, T>
}> {
    const store: ValidateBoardStore = {
        stack: initValidateBoardStack(),
    }
    const action = new Action(converter, {
        update: updateValidateBoardState(fields, store),
    })
    return {
        validate: action,
        checker: action,
    }
}

class Action<N extends string, T>
    extends ApplicationAbstractStateAction<ValidateBoardActionState>
    implements ValidateBoardAction, ValidateBoardChecker<N, T>
{
    readonly initialState: ValidateBoardActionState = initialValidateBoardState

    converter: BoardConverter<T>
    material: ValidateBoardMaterial<N>

    constructor(converter: BoardConverter<T>, material: ValidateBoardMaterial<N>) {
        super()
        this.converter = converter
        this.material = material
    }

    update(name: N, result: boolean): void {
        this.material.update(name, result, this.post)
    }
    get(): ConvertBoardResult<T> {
        return this.converter()
    }

    clear(): void {
        this.post(initialValidateBoardState)
    }
}
