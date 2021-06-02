import { ApplicationAbstractStateAction } from "../../../action/impl"

import { convertBoardField } from "../../validate_field/impl"

import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
    ValidateBoardFieldMaterial,
} from "./action"

import { ConvertBoardFieldResult } from "../../validate_field/data"
import { ValidateBoardFieldInfra } from "../../validate_field/infra"

export function initValidateBoardFieldAction<T, E>(
    infra: ValidateBoardFieldInfra<T, E>,
): ValidateBoardFieldAction<T, E> {
    return new Action({
        convert: convertBoardField(infra),
    })
}

class Action<T, E>
    extends ApplicationAbstractStateAction<ValidateBoardFieldState<E>>
    implements ValidateBoardFieldAction<T, E> {
    readonly initialState: ValidateBoardFieldState<E> = { valid: true }

    material: ValidateBoardFieldMaterial<T, E>

    constructor(material: ValidateBoardFieldMaterial<T, E>) {
        super()
        this.material = material
    }

    get(): ConvertBoardFieldResult<T, E> {
        return this.material.convert(this.post)
    }
    check(): Promise<ValidateBoardFieldState<E>> {
        return new Promise((resolve) => {
            this.material.convert((state) => resolve(this.post(state)))
        })
    }
}
