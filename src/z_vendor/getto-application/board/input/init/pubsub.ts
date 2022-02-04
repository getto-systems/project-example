import {
    InputBoardEventHandler,
    InputBoardEventPublisher,
    InputBoardEventSubscriber,
} from "../infra"

export function initInputEventPubSub(): Readonly<{
    publisher: InputBoardEventPublisher
    subscriber: InputBoardEventSubscriber
}> {
    let handlers: InputBoardEventHandler[] = []

    return {
        publisher: {
            post: () => {
                handlers.forEach((handler) => handler())
            },
        },
        subscriber: {
            subscribe: (handler) => {
                handlers = [...handlers, handler]
            },
            terminate: () => {
                handlers = []
            },
        },
    }
}
