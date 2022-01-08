import { initBoardValueStoreConnector } from "./init/connector"
import { initMultipleBoardValueStoreConnector } from "./init/multiple_connector"
import { initInputEventPubSub } from "./init/pubsub"

import {
    BoardValueStore,
    BoardValueStoreConnector,
    InputBoardEventPublisher,
    InputBoardEventSubscriber,
    MultipleBoardValueStore,
    MultipleBoardValueStoreConnector,
} from "./infra"

export interface InputBoardAction {
    // 例外的に infra をそのまま公開する
    // input を infra として使用するので、この要素は action よりも infra に近い
    readonly connector: BoardValueStoreConnector
    readonly publisher: InputBoardEventPublisher
}

export interface MultipleInputBoardAction {
    // 例外的に infra をそのまま公開する
    // input を infra として使用するので、この要素は action よりも infra に近い
    readonly connector: MultipleBoardValueStoreConnector
    readonly publisher: InputBoardEventPublisher
}

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
