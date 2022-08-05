import { initApplicationState, StatefulApplicationAction } from "../../action/action"

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
    const infra = {
        stack: initObserveBoardStack(),
    }
    const { state, post } = initApplicationState({ initialState })
    return {
        observe: { state, clear },
        observeChecker: { update },
    }

    function clear(): ObserveBoardState {
        return post(clearState(config, infra))
    }
    function update(name: N, hasChanged: boolean): ObserveBoardState {
        return post(updateState(config, infra, name, hasChanged))
    }
}

function clearState<N extends string>(
    config: ObserveBoardConfig<N>,
    infra: ObserveBoardInfra,
): ObserveBoardFieldResult {
    const { stack } = infra

    stack.clear()
    return compose(config.fields.map((field) => stack.get(field)))
}
function updateState<N extends string>(
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
