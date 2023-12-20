import { Atom, combine3Atom, combineAtom } from "./atom"

export function observeAtom<T>(state: Atom<T>): () => readonly T[] {
    const stack: T[] = []
    const handler = (state: T) => {
        stack.push(state)
    }

    state.subscribe(handler)

    return () => {
        state.unsubscribe(handler)
        return stack
    }
}

export function observe2Atom<A, B>(stateA: Atom<A>, stateB: Atom<B>): () => readonly [A, B][] {
    return observeAtom(combineAtom(stateA, stateB, (a, b) => [a, b]))
}

export function observe3Atom<A, B, C>(
    stateA: Atom<A>,
    stateB: Atom<B>,
    stateC: Atom<C>,
): () => readonly [A, B, C][] {
    return observeAtom(combine3Atom(stateA, stateB, stateC, (a, b, c) => [a, b, c]))
}
