import { ApplicationStateAction } from "../../action/action"
import { ApplicationAbstractStateAction } from "../../action/init"

import { ObserveBoardFieldResult } from "../observe_field/data"
import { ObserveBoardChecker, ObserveBoardStack, ObserveBoardStateFound } from "./infra"
import { initObserveBoardStack } from "./init/stack"

export type ObserveBoardAction = ApplicationStateAction<ObserveBoardActionState>

export type ObserveBoardActionState = ObserveBoardFieldResult
export const initialObserveBoardState: ObserveBoardActionState = { hasChanged: false }

export type ObserveBoardConfig<N extends string> = Readonly<{
    fields: readonly N[]
}>
export type ObserveBoardInfra = Readonly<{
    stack: ObserveBoardStack
}>

export function initObserveBoardAction<N extends string>(
    config: ObserveBoardConfig<N>,
): Readonly<{
    observe: ObserveBoardAction
    checker: ObserveBoardChecker<N>
}> {
    const action = new Action(config, {
        stack: initObserveBoardStack(),
    })
    return {
        observe: action,
        checker: action,
    }
}

class Action<N extends string>
    extends ApplicationAbstractStateAction<ObserveBoardActionState>
    implements ObserveBoardAction, ObserveBoardChecker<N>
{
    readonly initialState: ObserveBoardActionState = initialObserveBoardState

    config: ObserveBoardConfig<N>
    infra: ObserveBoardInfra

    constructor(config: ObserveBoardConfig<N>, infra: ObserveBoardInfra) {
        super()
        this.config = config
        this.infra = infra
    }

    update(name: N, hasChanged: boolean): ObserveBoardActionState {
        return this.post(update(this.config, this.infra, name, hasChanged))
    }
}

function update<N extends string>(
    config: ObserveBoardConfig<N>,
    infra: ObserveBoardInfra,
    name: N,
    hasChanged: boolean,
): ObserveBoardFieldResult {
    const { stack } = infra

    stack.set(name, hasChanged)
    return compose(config.fields.map((field) => stack.get(field)))
}

function compose(results: ObserveBoardStateFound[]): ObserveBoardFieldResult {
    if (results.some((result) => result.found && result.hasChanged)) {
        return { hasChanged: true }
    }
    return { hasChanged: false }
}
