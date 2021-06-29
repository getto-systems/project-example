import { ApplicationActionStateHandler, ApplicationActionStateSubscriber } from "../action"
import { ApplicationActionStatePublisher } from "../infra"

export type ApplicationActionStatePubSub<S> = Readonly<{
    pub: ApplicationActionStatePublisher<S>
    sub: ApplicationActionStateSubscriber<S>
}>
export function initActionStatePubSub<S>(): ApplicationActionStatePubSub<S> {
    return new PubSub<S>()
}

class PubSub<S> {
    handlers: ApplicationActionStateHandler<S>[] = []

    pub: ApplicationActionStatePublisher<S> = {
        post: (state) => {
            this.handlers.forEach((post) => post(state))
            return state
        },
        terminate: () => {
            this.handlers = []
        },
    }
    sub: ApplicationActionStateSubscriber<S> = {
        subscribe: (handler) => {
            this.handlers = [...this.handlers, handler]
        },
        unsubscribe: (target) => {
            this.handlers = this.handlers.filter((handler) => handler !== target)
        },
    }
}
