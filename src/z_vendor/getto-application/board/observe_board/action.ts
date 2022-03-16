import { StatefulApplicationAction, AbstractStatefulApplicationAction } from "../../action/action"

import { ObserveBoardFieldResult } from "../observe_field/data"
import { ObserveBoardChecker, ObserveBoardStack, ObserveBoardStateFound } from "./infra"
import { initObserveBoardStack } from "./init/stack"

export interface ObserveBoardAction extends StatefulApplicationAction<ObserveBoardState> {
    clear(): ObserveBoardState
}

export type ObserveBoardState = ObserveBoardFieldResult
const initialState: ObserveBoardState = { hasChanged: false }

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
    observeChecker: ObserveBoardChecker<N>
}> {
    const action = new Action(config, {
        stack: initObserveBoardStack(),
    })
    return {
        observe: action,
        observeChecker: action,
    }
}

class Action<N extends string>
    extends AbstractStatefulApplicationAction<ObserveBoardState>
    implements ObserveBoardAction, ObserveBoardChecker<N>
{
    readonly initialState = initialState

    config: ObserveBoardConfig<N>
    infra: ObserveBoardInfra

    constructor(config: ObserveBoardConfig<N>, infra: ObserveBoardInfra) {
        super()
        this.config = config
        this.infra = infra
    }

    clear(): ObserveBoardState {
        return this.post(clear(this.config, this.infra))
    }
    update(name: N, hasChanged: boolean): ObserveBoardState {
        return this.post(update(this.config, this.infra, name, hasChanged))
    }
}

function clear<N extends string>(
    config: ObserveBoardConfig<N>,
    infra: ObserveBoardInfra,
): ObserveBoardFieldResult {
    const { stack } = infra

    stack.clear()
    return compose(config.fields.map((field) => stack.get(field)))
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

function compose(results: readonly ObserveBoardStateFound[]): ObserveBoardFieldResult {
    if (results.some((result) => result.found && result.hasChanged)) {
        return { hasChanged: true }
    }
    return { hasChanged: false }
}
