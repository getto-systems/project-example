import { StatefulApplicationAction } from "./action"

export interface ApplicationActionTestRunner<S> {
    (statement: { (): Promise<S> }): Promise<S[]>
}

export function setupActionTestRunner<S>(
    action: StatefulApplicationAction<S>,
): ApplicationActionTestRunner<S> {
    return async (statement) => {
        const stack: S[] = []
        const handler = (state: S) => {
            stack.push(state)
        }

        action.state.subscribe(handler)
        await statement()

        action.state.unsubscribe(handler)
        return stack
    }
}
