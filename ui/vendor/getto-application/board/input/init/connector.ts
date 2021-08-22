import { BoardValueStore, BoardValueStoreConnector } from "../infra"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

export function initBoardValueStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector
    store: BoardValueStore
}> {
    const connector = new Connector()
    return {
        connector,
        store: connector,
    }
}

class Connector implements BoardValueStoreConnector, BoardValueStore {
    conn: BoardValueStoreConnection = { connect: false }

    get(): BoardValue {
        if (!this.conn.connect) {
            return emptyBoardValue
        }
        return this.conn.store.get()
    }
    set(value: BoardValue): void {
        if (this.conn.connect) {
            this.conn.store.set(value)
        }
    }

    connect(store: BoardValueStore): void {
        this.conn = { connect: true, store }
    }
    terminate(): void {
        this.conn = { connect: false }
    }
}

type BoardValueStoreConnection =
    | Readonly<{ connect: false }>
    | Readonly<{ connect: true; store: BoardValueStore }>
