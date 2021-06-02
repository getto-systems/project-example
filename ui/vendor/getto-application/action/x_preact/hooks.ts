import { useEffect, useLayoutEffect, useState } from "preact/hooks"

import { ApplicationView, ApplicationStateAction } from "../action"

export function useApplicationView<R>({ resource, terminate }: ApplicationView<R>): R {
    useEffect(() => terminate, [terminate])
    return resource
}

export function useApplicationAction<S>(action: ApplicationStateAction<S>): S {
    const [state, setState] = useState(action.initialState)
    useLayoutEffect(() => {
        action.subscriber.subscribe(setState)
        action.ignite()
        return () => {
            action.subscriber.unsubscribe(setState)
        }
    }, [action])
    return state
}
