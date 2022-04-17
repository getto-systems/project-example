import {
    initBoardValueStoreConnector,
    initFileStoreConnector,
    initMultipleBoardValueStoreConnector,
} from "./init/connector"
import { initInputEventPubSub } from "./init/pubsub"

import {
    BoardValueStore,
    BoardValueStoreConnector,
    FileStore,
    InputBoardEventPublisher,
    InputBoardEventSubscriber,
    MultipleBoardValueStore,
} from "./infra"

export interface InputBoardAction<S> {
    // 例外的に infra をそのまま公開する
    // input を infra として使用するので、この要素は action よりも infra に近い
    readonly connector: BoardValueStoreConnector<S>
    readonly publisher: InputBoardEventPublisher
}

export function initInputBoardAction(): Readonly<{
    input: InputBoardAction<BoardValueStore>
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
    input: InputBoardAction<MultipleBoardValueStore>
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

export function initSelectFileAction(): Readonly<{
    input: InputBoardAction<FileStore>
    store: FileStore
    subscriber: InputBoardEventSubscriber
}> {
    const { connector, store } = initFileStoreConnector()
    const { publisher, subscriber } = initInputEventPubSub()

    return {
        input: { connector, publisher },
        store,
        subscriber,
    }
}
