import { useLayoutEffect, useState } from "preact/hooks"

import { StatefulApplicationAction } from "../action"

export function useApplicationAction<S>(action: StatefulApplicationAction<S>): S {
    const [state, setState] = useState(action.state.currentState())
    useLayoutEffect(() => {
        action.state.subscribe(setState)
        return () => {
            action.state.unsubscribe(setState)
        }
    }, [action])
    return state
}
export function useApplicationActionWithFallback<S>(
    action: StatefulApplicationAction<S> | undefined,
    fallbackState: S,
): S {
    const [state, setState] = useState(
        action === undefined ? fallbackState : action.state.currentState(),
    )
    useLayoutEffect(() => {
        if (action) {
            action.state.subscribe(setState)
            return () => {
                action.state.unsubscribe(setState)
            }
        }
    }, [action])
    return state
}
