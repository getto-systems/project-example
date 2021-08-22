import { initBoardValueStoreConnector } from "../input/init/connector"
import { initInputEventPubSub } from "../input/init/pubsub"

import { InputBoardAction } from "./action"

import { BoardValueStore, InputBoardEventSubscriber } from "../input/infra"

export function initInputBoardAction(): Readonly<{
    input: InputBoardAction
    store: BoardValueStore
    subscriber: InputBoardEventSubscriber
}> {
    const { connector, store } = initBoardValueStoreConnector()
    const { publisher, subscriber } = initInputEventPubSub()

    return {
        input: { connector, publisher },
        store,
        subscriber,
    }
}
