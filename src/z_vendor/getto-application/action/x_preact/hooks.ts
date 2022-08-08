import { useLayoutEffect, useState } from "preact/hooks"

import { ApplicationState } from "../action"

export function useApplicationState<S>(state: ApplicationState<S>): S {
    const [currentState, setState] = useState(state.currentState())
    useLayoutEffect(() => {
        state.subscribe(setState)
        return () => {
            state.unsubscribe(setState)
        }
    }, [state])
    return currentState
}
export function useApplicationStateWithFallback<S>(
    state: ApplicationState<S> | undefined,
    fallbackState: S,
): S {
    const [currentState, setState] = useState(
        state === undefined ? fallbackState : state.currentState(),
    )
    useLayoutEffect(() => {
        if (state) {
            state.subscribe(setState)
            return () => {
                state.unsubscribe(setState)
            }
        }
    }, [state])
    return currentState
}
