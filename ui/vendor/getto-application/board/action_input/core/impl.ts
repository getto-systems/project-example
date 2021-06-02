import { BoardValueStore } from "../../input/infra"

import { BoardValueStoreLinker, InputBoardValueAction, InputBoardValueHandler } from "./action"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

export function initInputBoardValueAction(): InputBoardValueAction {
    return new Action()
}

class Action implements InputBoardValueAction {
    link: BoardValueStoreLink = { connect: false }

    inputEvent: InputEventPubSub = initInputEventPubSub()

    readonly storeLinker: BoardValueStoreLinker = {
        link: (store: BoardValueStore) => {
            this.link = { connect: true, store }
        },
        unlink: () => {
            this.link = { connect: false }
        },
    }

    subscribeInputEvent(handler: InputBoardValueHandler): void {
        this.inputEvent.subscribe(handler)
    }
    triggerInputEvent(): void {
        this.inputEvent.trigger()
    }

    get(): BoardValue {
        if (!this.link.connect) {
            return emptyBoardValue
        }
        return this.link.store.get()
    }
    set(value: BoardValue): void {
        if (this.link.connect) {
            this.link.store.set(value)
        }
        this.triggerInputEvent()
    }
    clear(): void {
        this.set(emptyBoardValue)
    }

    terminate(): void {
        this.storeLinker.unlink()
        this.inputEvent.terminate()
    }
}

type BoardValueStoreLink =
    | Readonly<{ connect: false }>
    | Readonly<{ connect: true; store: BoardValueStore }>

interface InputEventPubSub {
    subscribe(handler: InputBoardValueHandler): void
    trigger(): void
    terminate(): void
}

function initInputEventPubSub(): InputEventPubSub {
    let handlers: InputBoardValueHandler[] = []

    return {
        subscribe: (handler) => {
            handlers = [...handlers, handler]
        },
        trigger: () => {
            handlers.forEach((handler) => handler())
        },
        terminate: () => {
            handlers = []
        },
    }
}
