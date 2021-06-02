import { ApplicationActionStateSubscriber } from "./action"

export interface ApplicationActionTestRunner<S> {
    (statement: { (): Promise<S> }): Promise<S[]>
}

export function setupActionTestRunner<S>(
    subscriber: ApplicationActionStateSubscriber<S>,
): ApplicationActionTestRunner<S> {
    return async (statement) => {
        const stack: S[] = []
        const handler = (state: S) => {
            stack.push(state)
        }

        subscriber.subscribe(handler)
        await statement()

        subscriber.unsubscribe(handler)
        return stack
    }
}
