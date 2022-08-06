import { ApplicationState } from "./action"

export async function observeApplicationState<S>(
    state: ApplicationState<S>,
    statement: () => Promise<S>,
): Promise<S[]> {
    const stack: S[] = []
    const handler = (state: S) => {
        stack.push(state)
    }

    state.subscribe(handler)

    await statement()

    state.unsubscribe(handler)
    return stack
}

export async function observeApplicationStateTuple2<S1, S2>(
    state: [ApplicationState<S1>, ApplicationState<S2>],
    statement: () => Promise<[S1, S2]>,
): Promise<[readonly S1[], readonly S2[]]> {
    const stack: [S1[], S2[]] = [[], []]
    const handlers: [(state: S1) => void, (state: S2) => void] = [
        (state) => {
            stack[0].push(state)
        },
        (state) => {
            stack[1].push(state)
        },
    ]

    state[0].subscribe(handlers[0])
    state[1].subscribe(handlers[1])

    await statement()

    state[0].unsubscribe(handlers[0])
    state[1].unsubscribe(handlers[1])
    return stack
}
