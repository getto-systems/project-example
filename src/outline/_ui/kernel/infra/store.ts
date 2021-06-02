import { FetchMenuStoreResult, MenuBadgeStore, MenuExpandStore, MenuStore } from "../infra"

export function initMenuExpandStore(): MenuExpandStore {
    return initMenuStore()
}
export function initMenuBadgeStore(): MenuBadgeStore {
    return initMenuStore()
}

function initMenuStore<T>(): MenuStore<T> {
    let stored: FetchMenuStoreResult<T> = { found: false }
    return {
        get: () => stored,
        set: (value) => {
            stored = { found: true, value }
        },
    }
}
