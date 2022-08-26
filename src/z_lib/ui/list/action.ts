import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"
import { ModifyFieldHandler } from "../modify/action"

import { PrepareElementState } from "../prepare/data"
import { ScrollPosition } from "../scroll/data"
import { DetectFocusListKeyResult, ListSearchedResult } from "./data"

export interface ListRegisteredAction<T> extends ListAction<readonly T[]> {
    readonly focus: FocusRegisteredAction<T>
}

export interface ListSearchedAction<T, M, E> extends ListAction<ListSearchedResult<T, M, E>> {
    readonly focus: FocusSearchedAction<T>
    readonly scroll: ScrollAction
}

interface ListAction<S> {
    readonly state: ApplicationState<ListState<S>>
}

type ListState<S> = PrepareElementState<S>

export interface FocusRegisteredAction<T> {
    readonly state: ApplicationState<FocusState<T>>

    onModify(handler: ModifyFieldHandler<T>): void

    change(data: T): FocusState<T>
    update(data: Partial<T>): FocusState<T>
    remove(): FocusState<T>
    close(): FocusState<T>
}

export interface FocusSearchedAction<T> {
    readonly state: ApplicationState<FocusState<T>>

    onModify(handler: ModifyFieldHandler<T>): void

    change(data: T, position: ScrollPosition): FocusState<T>
    update(data: Partial<T>): FocusState<T>
    remove(): FocusState<T>
    close(position: ScrollPosition): FocusState<T>
}

export type FocusState<T> =
    | Readonly<{ type: "close"; isFocused: false }>
    | Readonly<{ type: "close"; isFocused: true; data: T }>
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "focus-change"; data: T }>
    | Readonly<{ type: "data-update"; data: T }>
    | Readonly<{ type: "data-remove" }>

export type FocusedData<T> = Readonly<{ isFocused: false }> | Readonly<{ isFocused: true; data: T }>

export function focusedData<T>(state: FocusState<T>): FocusedData<T> {
    switch (state.type) {
        case "close":
            return state.isFocused ? { isFocused: true, data: state.data } : { isFocused: false }

        case "not-found":
        case "data-remove":
            return { isFocused: false }

        case "focus-change":
        case "data-update":
            return { isFocused: true, data: state.data }
    }
}

export interface ScrollAction {
    readonly state: ApplicationState<ScrollState>
}

export type ScrollState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "detect" }>
    | Readonly<{ type: "focus-change"; position: ScrollPosition }>
    | Readonly<{ type: "close"; position: ScrollPosition }>

export interface ListRegisteredHandler<T> {
    register(data: T): ListState<readonly T[]>
}

export function initListRegisteredAction<T>(): Readonly<{
    action: ListRegisteredAction<T>
    handler: ListRegisteredHandler<T>
}> {
    const { action, handler, mutate } = initListAction<T, readonly T[]>({
        list: (state) => ({ isLoad: true, data: state }),
        mutate: (_state, list) => list,
    })
    const focus = initFocusRegistered<T>({ list: handler })

    return {
        action: { ...action, focus: focus.action },
        handler: { register },
    }

    function register(data: T): ListState<readonly T[]> {
        return mutate((element: ListState<readonly T[]>): ListState<readonly T[]> => {
            return {
                isLoad: true,
                data: element.isLoad ? [data, ...element.data] : [data],
            }
        })
    }
}

export type ListSearchedActionProps<T, M, E> = Readonly<{
    initialSearch: Promise<PrepareElementState<ListSearchedResult<T, M, E>>>
    detect: Readonly<{
        get: () => DetectFocusListKeyResult
        key: (data: T) => string
    }>
}>

export interface ListSearchedHandler<T, M, E> {
    load(data: ListState<ListSearchedResult<T, M, E>>): void
}

export function initListSearchedAction<T, M, E>(
    props: ListSearchedActionProps<T, M, E>,
): Readonly<{
    action: ListSearchedAction<T, M, E>
    handler: ListSearchedHandler<T, M, E>
}> {
    const { action, handler, mutate } = initListAction<T, ListSearchedResult<T, M, E>>({
        ignite: detect,
        list: (state) => {
            switch (state.type) {
                case "success":
                    return { isLoad: true, data: state.response.list }

                case "failed":
                    return { isLoad: false }
            }
        },
        mutate: (state, list) => {
            switch (state.type) {
                case "success":
                    return {
                        ...state,
                        response: { ...state.response, list },
                    }

                case "failed":
                    return state
            }
        },
    })

    const scroll = initScroll()
    const focus = initFocusSearched<T>({
        list: handler,
        scroll: scroll.handler,
    })

    return {
        action: {
            ...action,
            focus: focus.action,
            scroll: scroll.action,
        },
        handler: { load },
    }

    async function detect(): Promise<ListState<ListSearchedResult<T, M, E>>> {
        await props.initialSearch
        const search = action.state.currentState()
        if (search.isLoad && search.data.type === "success") {
            const detected = props.detect.get()
            if (detected.found) {
                const item = search.data.response.list.find(
                    (item) => props.detect.key(item) === detected.key,
                )
                focus.detect(item)
            }
        }
        return search
    }

    function load(
        data: ListState<ListSearchedResult<T, M, E>>,
    ): ListState<ListSearchedResult<T, M, E>> {
        return mutate(() => data)
    }
}

type ListProps<T, S> = Readonly<{
    ignite?: () => Promise<ListState<S>>
    list: (state: S) => PrepareElementState<readonly T[]>
    mutate: (state: S, list: readonly T[]) => S
}>
interface ListHandler<T> {
    find(data: T): boolean
    update(oldData: T, newData: T): void
    remove(data: T): void
}
function initListAction<T, S>(
    props: ListProps<T, S>,
): Readonly<{
    action: ListAction<S>
    handler: ListHandler<T>
    mutate: (f: (element: ListState<S>) => ListState<S>) => ListState<S>
}> {
    let element: ListState<S> = { isLoad: false }

    const { state, post } = initApplicationState<ListState<S>>({
        initialState: element,
        ignite: props.ignite,
    })

    return {
        action: { state },
        handler: { find, update, remove },
        mutate: (f) => {
            element = f(element)
            return post(element)
        },
    }

    function find(data: T): boolean {
        if (!element.isLoad) {
            return false
        }

        const list = props.list(element.data)
        if (!list.isLoad) {
            return false
        }

        return list.data.find((item) => item === data) !== undefined
    }

    function update(oldData: T, newData: T): ListState<S> {
        if (!element.isLoad) {
            return element
        }

        const list = props.list(element.data)
        if (!list.isLoad) {
            return element
        }

        element = {
            isLoad: true,
            data: props.mutate(
                element.data,
                list.data.map((item) => (item === oldData ? newData : item)),
            ),
        }

        return post(element)
    }
    function remove(data: T): ListState<S> {
        if (!element.isLoad) {
            return element
        }

        const list = props.list(element.data)
        if (!list.isLoad) {
            return element
        }

        element = {
            isLoad: true,
            data: props.mutate(
                element.data,
                list.data.filter((item) => item !== data),
            ),
        }

        return post(element)
    }
}

interface ScrollHandler {
    detect(): void
    change(position: ScrollPosition): void
    close(position: ScrollPosition): void
}

function initScroll(): Readonly<{
    action: ScrollAction
    handler: ScrollHandler
}> {
    const { state, post } = initApplicationState<ScrollState>({
        initialState: { type: "initial" },
    })

    return {
        action: {
            state,
        },
        handler: {
            detect(): ScrollState {
                return post({ type: "detect" })
            },
            change(position): ScrollState {
                return post({ type: "focus-change", position })
            },
            close(position): ScrollState {
                return post({ type: "close", position })
            },
        },
    }
}

type FocusRegisteredProps<T> = Readonly<{
    list: ListHandler<T>
}>

function initFocusRegistered<T>(props: FocusRegisteredProps<T>): Readonly<{
    action: FocusRegisteredAction<T>
    detect(data: T | undefined): void
}> {
    return initFocus(props)
}

type FocusSearchedProps<T> = Readonly<{
    list: ListHandler<T>
    scroll: ScrollHandler
}>

function initFocusSearched<T>(props: FocusSearchedProps<T>): Readonly<{
    action: FocusSearchedAction<T>
    detect(data: T | undefined): void
}> {
    return initFocus(props)
}

type FocusProps<T> = Readonly<{
    list: ListHandler<T>
    scroll?: ScrollHandler
}>

function initFocus<T>(props: FocusProps<T>) {
    const { state, post } = initApplicationState<FocusState<T>>({
        initialState: { type: "close", isFocused: false },
    })

    let element: PrepareElementState<T> = { isLoad: false }

    return {
        action: { state, onModify, change, update, remove, close },
        detect,
    }

    function onModify(handler: ModifyFieldHandler<T>): void {
        state.subscribe((state): true => {
            switch (state.type) {
                case "focus-change":
                    handler.focus(state.data)
                    return true

                case "data-update":
                    handler.update(state.data)
                    return true

                case "close":
                    handler.close()
                    return true

                case "not-found":
                case "data-remove":
                    return true
            }
        })
    }

    function detect(data: T | undefined): FocusState<T> {
        if (data === undefined) {
            return post({ type: "not-found" })
        } else {
            element = { isLoad: true, data }
            props.scroll?.detect()
            return post({ type: "focus-change", data })
        }
    }

    function change(data: T, position?: ScrollPosition): FocusState<T> {
        if (!props.list.find(data)) {
            return not_found()
        }

        element = { isLoad: true, data }
        if (position !== undefined) {
            props.scroll?.change(position)
        }
        return post({ type: "focus-change", data })
    }
    function update(data: Partial<T>): FocusState<T> {
        if (!element.isLoad) {
            return state.currentState()
        }

        const newData = { ...element.data, ...data }
        props.list.update(element.data, newData)

        element = { isLoad: true, data: newData }
        return post({ type: "data-update", data: newData })
    }
    function remove(): FocusState<T> {
        if (element.isLoad) {
            props.list.remove(element.data)
        }

        element = { isLoad: false }
        return post({ type: "data-remove" })
    }
    function close(position?: ScrollPosition): FocusState<T> {
        const focused: { isFocused: false } | { isFocused: true; data: T } = element.isLoad
            ? { isFocused: true, data: element.data }
            : { isFocused: false }

        element = { isLoad: false }
        if (position !== undefined) {
            props.scroll?.close(position)
        }
        return post({ type: "close", ...focused })
    }
    function not_found(): FocusState<T> {
        element = { isLoad: false }
        return post({ type: "not-found" })
    }
}
