import { initBoardValueStoreConnector } from "../input/init/connector"
import { initInputEventPubSub } from "../input/init/pubsub"

import { InputBoardAction, MultipleInputBoardAction } from "./action"

import { BoardValueStore, InputBoardEventSubscriber, MultipleBoardValueStore } from "../input/infra"
import { initMultipleBoardValueStoreConnector } from "../input/init/multiple_connector"

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

export function initMultipleInputBoardAction(): Readonly<{
    input: MultipleInputBoardAction
    store: MultipleBoardValueStore
    subscriber: InputBoardEventSubscriber
}> {
    const { connector, store } = initMultipleBoardValueStoreConnector()
    const { publisher, subscriber } = initInputEventPubSub()

    return {
        input: { connector, publisher },
        store,
        subscriber,
    }
}
