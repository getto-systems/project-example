import { initApplicationState, StatefulApplicationAction } from "../../action/action"

import { initValidateBoardStack } from "./init/stack"

import {
    ValidateBoardChecker,
    ValidateBoardCheckState,
    ValidateBoardStack,
    ValidateBoardStateFound,
} from "./infra"

import { ConvertBoardResult } from "../kernel/data"

export interface ValidateBoardAction extends StatefulApplicationAction<ValidateBoardState> {
    clear(): ValidateBoardState
}

export type ValidateBoardState = "initial" | "valid" | "invalid"

const initialState: ValidateBoardState = "initial"

export type ValidateBoardConfig<N extends string> = Readonly<{
    fields: readonly N[]
}>
export type ValidateBoardInfra = Readonly<{
    stack: ValidateBoardStack
}>
export type ValidateBoardShell<T> = Readonly<{
    convert: { (): ConvertBoardResult<T> }
}>

export function initValidateBoardAction<N extends string, T>(
    config: ValidateBoardConfig<N>,
    shell: ValidateBoardShell<T>,
): Readonly<{
    validate: ValidateBoardAction
    validateChecker: ValidateBoardChecker<N, T>
}> {
    const infra = {
        stack: initValidateBoardStack(),
    }
    const { state, post } = initApplicationState({ initialState })
    return {
        validate: { state, clear },
        validateChecker: { update, get },
    }

    function update(name: N, state: ValidateBoardCheckState): ValidateBoardState {
        return post(updateState(config, infra, name, state))
    }
    function get(): ConvertBoardResult<T> {
        return shell.convert()
    }
    function clear(): ValidateBoardState {
        return post(initialState)
    }
}

function updateState<N extends string>(
    config: ValidateBoardConfig<N>,
    infra: ValidateBoardInfra,
    name: N,
    state: ValidateBoardCheckState,
): ValidateBoardState {
    const { stack } = infra

    switch (state.type) {
        case "initial":
            stack.delete(name)
            break

        case "validated":
            stack.set(name, state.result.valid)
            break
    }
    return compose(config.fields.map((field) => stack.get(field)))
}

function compose(results: readonly ValidateBoardStateFound[]): ValidateBoardState {
    if (results.some((result) => result.found && !result.state)) {
        return "invalid"
    }
    if (results.some((result) => !result.found)) {
        return "initial"
    }
    return "valid"
}
