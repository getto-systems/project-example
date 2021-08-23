import { ApplicationAbstractStateAction } from "../../action/init"

import { convertBoardField } from "../validate_field/method"

import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
    ValidateBoardFieldMaterial,
} from "./action"

import { ConvertBoardFieldResult } from "../validate_field/data"
import { BoardFieldChecker, ValidateBoardFieldInfra } from "../validate_field/infra"

export function initValidateBoardFieldAction<T, E>(
    infra: ValidateBoardFieldInfra<T, E>,
): Readonly<{ validate: ValidateBoardFieldAction<E>; checker: BoardFieldChecker<T, E> }> {
    const action = new Action({
        convert: convertBoardField(infra),
    })
    return {
        validate: action,
        checker: action,
    }
}

class Action<T, E>
    extends ApplicationAbstractStateAction<ValidateBoardFieldState<E>>
    implements ValidateBoardFieldAction<E>, BoardFieldChecker<T, E>
{
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

    clear(): void {
        this.post({ valid: true })
    }
}
