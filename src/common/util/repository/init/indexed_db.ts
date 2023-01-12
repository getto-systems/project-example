import { repositoryError } from "./helper"

import { FetchRepositoryResult, RepositoryErrorResult, StoreRepositoryResult } from "../infra"

// objectStore(`name`, { keyPath: "key" }) に { key: string, value: string } という値を保存する
export interface IndexedDB {
    get<T>(store: IndexedDBTarget, converter: FromDB<T>): Promise<FetchRepositoryResult<T>>
    set<T>(store: IndexedDBTarget, converter: ToDB<T>, value: T): Promise<StoreRepositoryResult>
    remove(store: IndexedDBTarget): Promise<StoreRepositoryResult>
}

export interface ToDB<T> {
    (value: T): string
}
export interface FromDB<T> {
    (raw: string): T
}

// データベース名と作成する objectStore を指定する
export type IndexedDBConfig = Readonly<{
    database: string
    stores: readonly string[]
}>

export type IndexedDBTarget = Readonly<{
    store: string
    key: string
}>

interface Migration {
    (db: IDBDatabase, stores: readonly string[]): void
}

// 構造を変えるときは migration を追加することで対応
const MIGRATIONS: readonly Migration[] = [
    (db, stores) => {
        stores.forEach((store) => {
            db.createObjectStore(store, { keyPath: "key" })
        })
    },
]

export function initIndexedDB(webDB: IDBFactory, config: IndexedDBConfig): IndexedDB {
    return {
        get<T>(target: IndexedDBTarget, converter: FromDB<T>): Promise<FetchRepositoryResult<T>> {
            return new Promise((resolve) => {
                open(resolve, (db) => {
                    try {
                        const tx = db.transaction(target.store)
                        tx.oncomplete = () => db.close()

                        const request = tx.objectStore(target.store).get(target.key)
                        request.onsuccess = (e: Event) => {
                            if (!e.target || !(e.target instanceof IDBRequest)) {
                                resolve(repositoryError("invalid get result"))
                                return
                            }
                            if (!e.target.result) {
                                resolve({ success: true, found: false })
                                return
                            }

                            try {
                                // e.target.result は any のため、実行時エラーを覚悟する
                                // ブラウザのオブジェクトストレージの内容が any なのは本質的で避けられない
                                resolve({
                                    success: true,
                                    found: true,
                                    value: converter(e.target.result.value),
                                })
                            } catch (err) {
                                resolve(repositoryError(`${err}`))
                            }
                        }
                        request.onerror = () => {
                            resolve(repositoryError("failed to get"))
                        }
                    } catch (err) {
                        resolve(repositoryError(`${err}`))
                    }
                })
            })
        },

        set<T>(
            target: IndexedDBTarget,
            converter: ToDB<T>,
            value: T,
        ): Promise<StoreRepositoryResult> {
            return new Promise((resolve) => {
                open(resolve, (db) => {
                    try {
                        const tx = db.transaction(target.store, "readwrite")
                        tx.oncomplete = () => db.close()

                        const request = tx
                            .objectStore(target.store)
                            .put({ key: target.key, value: converter(value) })
                        request.onsuccess = () => {
                            resolve({ success: true })
                        }
                        request.onerror = () => {
                            resolve(repositoryError("failed to put"))
                        }
                    } catch (err) {
                        resolve(repositoryError(`${err}`))
                    }
                })
            })
        },

        remove(target: IndexedDBTarget): Promise<StoreRepositoryResult> {
            return new Promise((resolve) => {
                open(resolve, (db) => {
                    try {
                        const tx = db.transaction(target.store, "readwrite")
                        tx.oncomplete = () => db.close()

                        const request = tx.objectStore(target.store).delete(target.key)
                        request.onsuccess = () => {
                            resolve({ success: true })
                        }
                        request.onerror = () => {
                            resolve(repositoryError("failed to remove"))
                        }
                    } catch (err) {
                        resolve(repositoryError(`${err}`))
                    }
                })
            })
        },
    }

    function open(error: Post<RepositoryErrorResult>, success: Post<IDBDatabase>): void {
        const request = webDB.open(config.database, MIGRATIONS.length)
        request.onupgradeneeded = upgrade(config.stores, MIGRATIONS)
        request.onerror = () => {
            error(repositoryError("failed to open db"))
        }
        request.onsuccess = (e: Event) => {
            if (!e.target || !(e.target instanceof IDBOpenDBRequest)) {
                error(repositoryError("invalid open db result"))
                return
            }
            success(e.target.result)
        }

        function upgrade(
            stores: readonly string[],
            migrations: readonly Migration[],
        ): { (e: IDBVersionChangeEvent): void } {
            return (e) => {
                if (!e.target || !(e.target instanceof IDBOpenDBRequest)) {
                    return
                }
                const db = e.target.result
                migrations
                    .slice(e.oldVersion, e.newVersion === null ? undefined : e.newVersion)
                    .forEach((migration) => migration(db, stores))
            }
        }
    }
}

interface Post<T> {
    (result: T): void
}
