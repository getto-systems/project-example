export interface Atom<S> {
    readonly ignitionState: Promise<S>
    subscribe(handler: AtomHandler<S>): void
    unsubscribe(target: AtomHandler<S>): void
    currentState(): S
}

export type AtomConfig<S> = Readonly<{
    state: Atom<S>
    post: AtomPublisher<S>
}>

export interface AtomHandler<S> {
    (state: S): void
}

export interface AtomPublisher<S> {
    (state: S): S
}

export function initAtom<S>(
    props: Readonly<{
        initialState: S
        ignite?: () => Promise<S>
    }>,
): AtomConfig<S> {
    let state = props.initialState
    let handlers: AtomHandler<S>[] = []

    return {
        state: {
            ignitionState: new Promise((resolve) => {
                // コンストラクタが重くならないように初期 action は非同期で呼び出す
                // 状態は currentState() で最新のものを参照するので subscribe を待つ必要はない
                setTimeout(async () => {
                    resolve(props.ignite ? await props.ignite() : props.initialState)
                }, 0)
            }),

            subscribe(handler: AtomHandler<S>): void {
                handlers.push(handler)
            },
            unsubscribe(target: AtomHandler<S>): void {
                handlers = handlers.filter((handler) => handler !== target)
            },

            currentState(): S {
                return state
            },
        },
        post,
    }

    function post(newState: S): S {
        state = newState
        handlers.forEach((handler) => handler(newState))
        return newState
    }
}

export function mapAtomStateful<T, M>(source: Atom<T>, mapper: (state: T) => M): Atom<M> {
    let handlers: {
        original: (state: M) => void
        mapped: (state: T) => void
    }[] = []
    let lastState: Readonly<{ set: false }> | Readonly<{ set: true; state: M }> = { set: false }

    return {
        ignitionState: source.ignitionState.then(mapper),

        subscribe(handler: (state: M) => void): void {
            const mappedHandler = (state: T) => {
                const mappedState = mapper(state)
                if (!lastState.set || lastState.state !== mappedState) {
                    handler(mappedState)
                }
                lastState = { set: true, state: mappedState }
            }
            handlers.push({
                original: handler,
                mapped: mappedHandler,
            })
            source.subscribe(mappedHandler)
        },
        unsubscribe(target: (state: M) => void): void {
            const found = handlers.find((handler) => handler.original === target)
            if (found) {
                source.unsubscribe(found.mapped)
            }
            handlers = handlers.filter((handler) => handler.original !== target)
        },

        currentState(): M {
            return mapper(source.currentState())
        },
    }
}

export function mapAtom<T, M>(source: Atom<T>, mapper: (state: T) => M): Atom<M> {
    let handlers: {
        original: (state: M) => void
        mapped: (state: T) => void
    }[] = []

    return {
        ignitionState: source.ignitionState.then(mapper),

        subscribe(handler: (state: M) => void): void {
            const mappedHandler = (state: T) => {
                handler(mapper(state))
            }
            handlers.push({
                original: handler,
                mapped: mappedHandler,
            })
            source.subscribe(mappedHandler)
        },
        unsubscribe(target: (state: M) => void): void {
            const found = handlers.find((handler) => handler.original === target)
            if (found) {
                source.unsubscribe(found.mapped)
            }
            handlers = handlers.filter((handler) => handler.original !== target)
        },

        currentState(): M {
            return mapper(source.currentState())
        },
    }
}

export function combineAtom<A, B, M>(
    sourceA: Atom<A>,
    sourceB: Atom<B>,
    mapper: (stateA: A, stateB: B) => M,
): Atom<M> {
    let handlers: {
        original: (state: M) => void
        mappedA: (state: A) => void
        mappedB: (state: B) => void
    }[] = []

    return {
        ignitionState: Promise.all([sourceA.ignitionState, sourceB.ignitionState]).then(
            ([stateA, stateB]) => mapper(stateA, stateB),
        ),

        subscribe(handler: (state: M) => void): void {
            const mappedA = (state: A) => {
                handler(mapper(state, sourceB.currentState()))
            }
            const mappedB = (state: B) => {
                handler(mapper(sourceA.currentState(), state))
            }
            handlers.push({
                original: handler,
                mappedA,
                mappedB,
            })
            sourceA.subscribe(mappedA)
            sourceB.subscribe(mappedB)
        },
        unsubscribe(target: (state: M) => void): void {
            const found = handlers.find((handler) => handler.original === target)
            if (found) {
                sourceA.unsubscribe(found.mappedA)
                sourceB.unsubscribe(found.mappedB)
            }
            handlers = handlers.filter((handler) => handler.original !== target)
        },

        currentState(): M {
            return mapper(sourceA.currentState(), sourceB.currentState())
        },
    }
}

export function combine3Atom<A, B, C, M>(
    sourceA: Atom<A>,
    sourceB: Atom<B>,
    sourceC: Atom<C>,
    mapper: (stateA: A, stateB: B, stateC: C) => M,
): Atom<M> {
    let handlers: {
        original: (state: M) => void
        mappedA: (state: A) => void
        mappedB: (state: B) => void
        mappedC: (state: C) => void
    }[] = []

    return {
        ignitionState: Promise.all([
            sourceA.ignitionState,
            sourceB.ignitionState,
            sourceC.ignitionState,
        ]).then(([stateA, stateB, stateC]) => mapper(stateA, stateB, stateC)),

        subscribe(handler: (state: M) => void): void {
            const mappedA = (stateA: A) => {
                handler(mapper(stateA, sourceB.currentState(), sourceC.currentState()))
            }
            const mappedB = (stateB: B) => {
                handler(mapper(sourceA.currentState(), stateB, sourceC.currentState()))
            }
            const mappedC = (stateC: C) => {
                handler(mapper(sourceA.currentState(), sourceB.currentState(), stateC))
            }
            handlers.push({
                original: handler,
                mappedA,
                mappedB,
                mappedC,
            })
            sourceA.subscribe(mappedA)
            sourceB.subscribe(mappedB)
            sourceC.subscribe(mappedC)
        },
        unsubscribe(target: (state: M) => void): void {
            const found = handlers.find((handler) => handler.original === target)
            if (found) {
                sourceA.unsubscribe(found.mappedA)
                sourceB.unsubscribe(found.mappedB)
                sourceC.unsubscribe(found.mappedC)
            }
            handlers = handlers.filter((handler) => handler.original !== target)
        },

        currentState(): M {
            return mapper(sourceA.currentState(), sourceB.currentState(), sourceC.currentState())
        },
    }
}

export function composeAtom<T, M>(
    sourceArr: readonly Atom<T>[],
    mapper: (state: readonly T[]) => M,
): Atom<M> {
    let handlers: {
        original: (state: M) => void
        mapped: readonly ((state: T) => void)[]
    }[] = []

    return {
        ignitionState: Promise.all(sourceArr.map((source) => source.ignitionState)).then(mapper),

        subscribe(handler: (state: M) => void): void {
            const mapped = sourceArr.map((source, i) => {
                return (state: T): void => {
                    handler(mapper(mappedState(i, state)))
                }
            })
            handlers.push({
                original: handler,
                mapped,
            })

            sourceArr.forEach((source, i) => {
                source.subscribe(mapped[i])
            })

            function mappedState(index: number, state: T): readonly T[] {
                return sourceArr.map((source, i) => {
                    if (i === index) {
                        return state
                    } else {
                        return source.currentState()
                    }
                })
            }
        },
        unsubscribe(target: (state: M) => void): void {
            const found = handlers.find((handler) => handler.original === target)
            if (found) {
                sourceArr.forEach((source, i) => {
                    source.unsubscribe(found.mapped[i])
                })
            }
            handlers = handlers.filter((handler) => handler.original !== target)
        },

        currentState(): M {
            return mapper(sourceArr.map((source) => source.currentState()))
        },
    }
}
