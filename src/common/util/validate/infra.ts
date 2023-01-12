import { ValidateResult } from "./data"

export interface Validator<T, E> {
    (value: T): ValidateResult<E>
}
