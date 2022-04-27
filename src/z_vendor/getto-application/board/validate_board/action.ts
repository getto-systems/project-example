import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

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
    const action = new Action(
        config,
        {
            stack: initValidateBoardStack(),
        },
        shell,
    )
    return {
        validate: action,
        validateChecker: action,
    }
}

class Action<N extends string, T>
    extends AbstractStatefulApplicationAction<ValidateBoardState>
    implements ValidateBoardAction, ValidateBoardChecker<N, T>
{
    readonly initialState: ValidateBoardState = initialState

    config: ValidateBoardConfig<N>
    infra: ValidateBoardInfra
    shell: ValidateBoardShell<T>

    constructor(
        config: ValidateBoardConfig<N>,
        infra: ValidateBoardInfra,
        shell: ValidateBoardShell<T>,
    ) {
        super()
        this.config = config
        this.infra = infra
        this.shell = shell
    }

    update(name: N, state: ValidateBoardCheckState): ValidateBoardState {
        return this.post(update(this.config, this.infra, name, state))
    }
    get(): ConvertBoardResult<T> {
        return this.shell.convert()
    }

    clear(): ValidateBoardState {
        return this.post(initialState)
    }
}

function update<N extends string>(
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
