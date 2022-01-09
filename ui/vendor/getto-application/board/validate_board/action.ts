import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

import { initValidateBoardStack } from "./init/stack"

import { BoardConverter } from "../kernel/infra"
import { ValidateBoardChecker, ValidateBoardStack, ValidateBoardStateFound } from "./infra"

import { ConvertBoardResult } from "../kernel/data"
import { ValidateBoardState } from "./data"

export interface ValidateBoardAction extends StatefulApplicationAction<ValidateBoardActionState> {
    clear(): void
}

export type ValidateBoardActionState = ValidateBoardState
export const initialValidateBoardState: ValidateBoardActionState = "initial"

export type ValidateBoardConfig<N extends string> = Readonly<{
    fields: readonly N[]
}>
export type ValidateBoardInfra = Readonly<{
    stack: ValidateBoardStack
}>
export type ValidateBoardShell<T> = Readonly<{
    converter: BoardConverter<T>
}>

export function initValidateBoardAction<N extends string, T>(
    config: ValidateBoardConfig<N>,
    shell: ValidateBoardShell<T>,
): Readonly<{
    validate: ValidateBoardAction
    checker: ValidateBoardChecker<N, T>
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
        checker: action,
    }
}

class Action<N extends string, T>
    extends AbstractStatefulApplicationAction<ValidateBoardActionState>
    implements ValidateBoardAction, ValidateBoardChecker<N, T>
{
    readonly initialState: ValidateBoardActionState = initialValidateBoardState

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

    update(name: N, result: boolean): ValidateBoardState {
        return this.post(update(this.config, this.infra, name, result))
    }
    get(): ConvertBoardResult<T> {
        return this.shell.converter()
    }

    clear(): void {
        this.post(initialValidateBoardState)
    }
}

function update<N extends string>(
    config: ValidateBoardConfig<N>,
    infra: ValidateBoardInfra,
    name: N,
    valid: boolean,
): ValidateBoardState {
    const { stack } = infra

    stack.set(name, valid)
    return compose(config.fields.map((field) => stack.get(field)))
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
