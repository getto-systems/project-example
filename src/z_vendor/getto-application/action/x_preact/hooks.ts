import { useLayoutEffect, useState } from "preact/hooks"

import { StatefulApplicationAction } from "../action"

export function useApplicationAction<S>(action: StatefulApplicationAction<S>): S {
    const [state, setState] = useState(action.currentState())
    useLayoutEffect(() => {
        action.subscriber.subscribe(setState)
        return () => {
            action.subscriber.unsubscribe(setState)
        }
    }, [action])
    return state
}
export function useApplicationActionWithFallback<S>(
    action: StatefulApplicationAction<S> | undefined,
    fallbackState: S,
): S {
    const [state, setState] = useState(action === undefined ? fallbackState : action.currentState())
    useLayoutEffect(() => {
        if (action) {
            action.subscriber.subscribe(setState)
            return () => {
                action.subscriber.unsubscribe(setState)
            }
        }
    }, [action])
    return state
}
