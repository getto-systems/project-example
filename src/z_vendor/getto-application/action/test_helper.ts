import { ApplicationState } from "./action"

export interface ApplicationActionTestRunner<S> {
    (statement: { (): Promise<S> }): Promise<S[]>
}

export function setupActionTestRunner<S>(
    state: ApplicationState<S>,
): ApplicationActionTestRunner<S> {
    return async (statement) => {
        const stack: S[] = []
        const handler = (state: S) => {
            stack.push(state)
        }

        state.subscribe(handler)
        await statement()

        state.unsubscribe(handler)
        return stack
    }
}
