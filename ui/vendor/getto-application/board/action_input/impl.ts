import { initInputBoardValueAction } from "./core/impl"
import { initBoardValueStoreConnector } from "../input/init/connector"
import { initInputEventPubSub } from "../input/init/pubsub"

import { InputBoardAction, InputBoardValueResource } from "./action"

import {
    BoardValueStore,
    InputBoardEventPublisher,
    InputBoardEventSubscriber,
} from "../input/infra"

import { InputBoardValueType } from "../input/data"

export function initInputBoardValueResource(type: InputBoardValueType): InputBoardValueResource {
    return {
        type,
        input: initInputBoardValueAction(),
    }
}

export function initInputBoardAction(): Readonly<{
    input: InputBoardAction
    store: BoardValueStore
    publisher: InputBoardEventPublisher
    subscriber: InputBoardEventSubscriber
}> {
    const { connector, store } = initBoardValueStoreConnector()
    const { publisher, subscriber } = initInputEventPubSub()

    return {
        input: { connector, publisher },
        store,
        publisher,
        subscriber,
    }
}
