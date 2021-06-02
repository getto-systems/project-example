import { ValidateBoardStateFound, ValidateBoardStore } from "./infra"

import { UpdateBoardValidateStateMethod } from "./method"

import { ValidateBoardState } from "./data"

interface Update {
    <N extends string>(fields: N[], store: ValidateBoardStore): UpdateBoardValidateStateMethod<N>
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
