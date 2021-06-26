import { ValidateBoardStateFound, ValidateBoardStore } from "./infra"

import { ValidateBoardState } from "./data"

export interface UpdateBoardValidateStateMethod<N extends string> {
    (name: N, valid: boolean, post: Post<ValidateBoardState>): void
}

interface Update {
    <N extends string>(
        fields: readonly N[],
        store: ValidateBoardStore,
    ): UpdateBoardValidateStateMethod<N>
}
export const updateBoardValidateState: Update = (fields, infra) => (name, valid, post) => {
    const { stack } = infra

    stack.set(name, valid)
    post(compose(fields.map((field) => stack.get(field))))
}

function compose(results: ValidateBoardStateFound[]): ValidateBoardState {
    if (results.some((result) => result.found && !result.state)) {
        return "invalid"
    }
    if (results.some((result) => !result.found)) {
        return "initial"
    }
    return "valid"
}

interface Post<E> {
    (event: E): void
}
