import { useEffect, useLayoutEffect, useState } from "preact/hooks"

import { ApplicationView, StatefulApplicationAction } from "../action"

export function useApplicationView<R>({ resource, terminate }: ApplicationView<R>): R {
    useEffect(() => terminate, [terminate])
    return resource
}

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
    const [state, setState] = useState(action?.currentState() || fallbackState)
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
