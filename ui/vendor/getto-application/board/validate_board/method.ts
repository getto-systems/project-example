import { ValidateBoardState } from "./data"

export interface UpdateBoardValidateStateMethod<N extends string> {
    (name: N, valid: boolean, post: Post<ValidateBoardState>): void
}

interface Post<E> {
    (event: E): void
}
