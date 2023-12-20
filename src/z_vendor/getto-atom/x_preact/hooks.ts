import { useLayoutEffect, useState } from "preact/hooks"

import { Atom } from "../atom"

export function useAtom<S>(state: Atom<S>): S {
    const [currentState, setState] = useState(state.currentState())
    useLayoutEffect(() => {
        state.subscribe(setState)
        return () => {
            state.unsubscribe(setState)
        }
    }, [state])
    return currentState
}
export function useAtomWithFallback<S>(state: Atom<S> | undefined, fallbackState: S): S {
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
