import { ValidateBoardFieldInfra } from "./infra"

import { ConvertBoardFieldResult, ValidateBoardFieldResult } from "./data"

export interface ConvertBoardFieldMethod<T, E> {
    (post: Post<ValidateBoardFieldResult<E>>): ConvertBoardFieldResult<T, E>
}

interface Convert {
    <T, E>(infra: ValidateBoardFieldInfra<T, E>): ConvertBoardFieldMethod<T, E>
}
export const convertBoardField: Convert = (infra) => (post) => {
    const { converter } = infra
    const result = converter()
    if (result.valid) {
        post({ valid: true })
    } else {
        post(result)
    }
    return result
}

interface Post<E> {
    (event: E): void
}
