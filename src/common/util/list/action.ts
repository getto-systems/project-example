import {
    Atom,
    AtomConfig,
    combineAtom,
    initAtom,
    mapAtomStateful,
} from "../../../z_vendor/getto-atom/atom"
import { loadState_loaded, LoadState } from "../load/data"

import { ScrollPosition } from "../scroll/data"
import { DetectFocusListKeyResult } from "./data"

export interface LoadableListAtomUpdater<T> {
    update(updater: (list: readonly T[]) => readonly T[]): void
}

export function initLoadableListAtomUpdater<T>(
    list: AtomConfig<LoadState<readonly T[]>>,
): LoadableListAtomUpdater<T> {
    return {
        update: (updater: (list: readonly T[]) => readonly T[]) => {
            const data = list.state.currentState()
            if (data.isLoad) {
                list.post(loadState_loaded(updater(data.data)))
            }
        },
    }
}

export interface PushListAction<T> {
    push(entry: T): void
}

export function initPushListAction<T>(
    atom: AtomConfig<LoadState<readonly T[]>>,
): PushListAction<T> {
    return {
        push(data: T): void {
            const list = atom.state.currentState()
            atom.post(loadState_loaded(list.isLoad ? [...list.data, data] : [data]))
        },
    }
}

export type FocusRegisterListAction<T> = Readonly<{
    readonly state: Atom<LoadState<T>>
    readonly isSomeEntryFocused: Atom<boolean>
    readonly detect: Atom<DetectFocusListKeyResult>

    isEntryFocused(entry: T, key: DetectFocusListKeyResult): boolean
    focusTo(entry: T): void
    close(): void
}>

export function initFocusRegisterListAction<T>(
    list: Atom<LoadState<readonly T[]>>,
    key: (data: T) => string,
): FocusRegisterListAction<T> {
    const detect = initAtom<DetectFocusListKeyResult>({ initialState: { found: false } })

    const focus = combineAtom(list, detect.state, (list, detect): LoadState<T> => {
        if (!detect.found) {
            return { isLoad: false }
        }
        if (!list.isLoad) {
            return { isLoad: false }
        }
        const found = list.data.find((item) => key(item) === detect.key)
        if (found === undefined) {
            return { isLoad: false }
        }
        return { isLoad: true, data: found }
    })

    return {
        state: focus,
        isSomeEntryFocused: mapAtomStateful(focus, (focus) => focus.isLoad),
        detect: detect.state,

        isEntryFocused(entry: T, detect: DetectFocusListKeyResult): boolean {
            if (!detect.found) {
                return false
            }
            return key(entry) === detect.key
        },
        focusTo(entry: T): void {
            detect.post({ found: true, key: key(entry) })
        },
        close(): void {
            detect.post({ found: false })
        },
    }
}

export type FocusModifyListAction<T> = Readonly<{
    readonly state: Atom<LoadState<T>>
    readonly isSomeEntryFocused: Atom<boolean>
    readonly detect: Atom<DetectFocusListKeyResult>
    readonly scroll: Atom<ScrollState>

    isEntryFocused(entry: T, key: DetectFocusListKeyResult): boolean
    focusTo(entry: T, position: ScrollPosition): void
    close(position: ScrollPosition): void
}>

export type ScrollState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "detect" }>
    | Readonly<{ type: "focus-change"; position: ScrollPosition }>
    | Readonly<{ type: "close"; position: ScrollPosition }>

export interface FocusModifyListInfra {
    detect(): DetectFocusListKeyResult
    update(state: DetectFocusListKeyResult): void
}

export function initFocusModifyListAction<T>(
    list: Atom<LoadState<readonly T[]>>,
    key: (data: T) => string,
    infra: FocusModifyListInfra,
): FocusModifyListAction<T> {
    const initialFocus = infra.detect()

    const detect = initAtom({ initialState: initialFocus })
    detect.state.subscribe((state) => infra.update(state))

    const scroll = initAtom<ScrollState>({
        initialState: { type: initialFocus.found ? "detect" : "initial" },
    })

    const focus = combineAtom(list, detect.state, (list, detect): LoadState<T> => {
        if (!detect.found) {
            return { isLoad: false }
        }
        if (!list.isLoad) {
            return { isLoad: false }
        }
        const found = list.data.find((item) => key(item) === detect.key)
        if (found === undefined) {
            return { isLoad: false }
        }
        return { isLoad: true, data: found }
    })

    return {
        state: focus,
        isSomeEntryFocused: mapAtomStateful(focus, (focus) => focus.isLoad),
        detect: detect.state,
        scroll: scroll.state,

        isEntryFocused(entry: T, detect: DetectFocusListKeyResult): boolean {
            if (!detect.found) {
                return false
            }
            return key(entry) === detect.key
        },
        focusTo(entry: T, position: ScrollPosition): void {
            detect.post({ found: true, key: key(entry) })
            scroll.post({ type: "focus-change", position })
        },
        close(position: ScrollPosition): void {
            detect.post({ found: false })
            scroll.post({ type: "close", position })
        },
    }
}
