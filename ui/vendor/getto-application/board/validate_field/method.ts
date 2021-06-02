import { ConvertBoardFieldResult, ValidateBoardFieldResult } from "./data"

export interface ConvertBoardFieldMethod<T, E> {
    (post: Post<ValidateBoardFieldResult<E>>): ConvertBoardFieldResult<T, E>
}

interface Post<E> {
    (event: E): void
}
