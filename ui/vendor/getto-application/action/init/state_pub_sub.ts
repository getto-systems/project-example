import { ApplicationStateHandler, ApplicationStateSubscriber } from "../action"
import { ApplicationActionStatePublisher } from "../infra"

export type ApplicationActionStatePubSub<S> = Readonly<{
    pub: ApplicationActionStatePublisher<S>
    sub: ApplicationStateSubscriber<S>
}>
export function initActionStatePubSub<S>(): ApplicationActionStatePubSub<S> {
    return new PubSub<S>()
}

class PubSub<S> {
    handlers: ApplicationStateHandler<S>[] = []

    pub: ApplicationActionStatePublisher<S> = {
        post: (state) => {
            this.handlers.forEach((post) => post(state))
            return state
        },
        terminate: () => {
            this.handlers = []
        },
    }
    sub: ApplicationStateSubscriber<S> = {
        subscribe: (handler) => {
            this.handlers = [...this.handlers, handler]
        },
        unsubscribe: (target) => {
            this.handlers = this.handlers.filter((handler) => handler !== target)
        },
    }
}
