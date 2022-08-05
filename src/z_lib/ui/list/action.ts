import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"
import { ModifyFieldHandler } from "../modify/action"

import { PrepareElementState } from "../prepare/data"
import { DetectFocusListKeyResult, ListSearchedResult } from "./data"

export interface ListRegisteredAction<T> extends ListAction<readonly T[]> {
    readonly focus: FocusRegisteredAction<T>
}

export interface FocusRegisteredAction<T> extends FocusAction<T, FocusState<T>> {
    readonly state: ApplicationState<FocusState<T>>
}

export interface ListSearchedAction<T, M, E> extends ListAction<ListSearchedResult<T, M, E>> {
    readonly focus: FocusSearchedAction<T>
}

export interface FocusSearchedAction<T> extends FocusAction<T, FocusSearchedState<T>> {
    readonly state: ApplicationState<FocusSearchedState<T>>
}

export type FocusSearchedState<T> = FocusState<T> | FocusDetectState<T>

interface ListAction<S> {
    readonly state: ApplicationState<ListState<S>>
}
type ListState<S> = PrepareElementState<S>

export interface FocusAction<T, S> {
    onModify(handler: ModifyFieldHandler<T>): void

    change(data: T): FocusState<T> | S
    update(data: Partial<T>): FocusState<T> | S
    remove(): FocusState<T> | S
    close(): FocusState<T> | S

    isFocused(data: T): boolean
}

export type FocusState<T> =
    | Readonly<{ type: "close" }>
    | Readonly<{ type: "change"; data: T }>
    | Readonly<{ type: "update"; data: T }>

type FocusDetectState<T> =
    | Readonly<{ type: "detect"; data: T }>
    | Readonly<{ type: "detect-failed" }>

export interface ListRegisteredHandler<T> {
    register(data: T): ListState<readonly T[]>
}

export function initListRegisteredAction<T>(): Readonly<{
    action: ListRegisteredAction<T>
    handler: ListRegisteredHandler<T>
}> {
    const { action, handler, mutate } = initListAction<T, readonly T[]>({
        ignite: undefined,
        list: (state) => ({ isLoad: true, data: state }),
        mutate: (_state, list) => list,
    })
    const focus = initFocus<T, FocusState<T>>({
        modifyState: (state) => ({ type: "invoke", state }),
        handler,
    })

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

    const focus = initFocus<T, FocusSearchedState<T>>({
        modifyState: (state) => {
            switch (state.type) {
                case "detect":
                    return { type: "invoke", state: { type: "change", data: state.data } }

                case "detect-failed":
                    return { type: "noop" }

                default:
                    return { type: "invoke", state }
            }
        },
        handler,
    })

    return {
        action: { ...action, focus: focus.action },
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
                focus.handler.store({
                    data: item,
                    state:
                        item === undefined
                            ? { type: "detect-failed" }
                            : { type: "detect", data: item },
                })
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

type FocusProps<T, S> = Readonly<{
    modifyState: (
        state: FocusState<T> | S,
    ) => Readonly<{ type: "noop" }> | Readonly<{ type: "invoke"; state: FocusState<T> }>
    handler: ListHandler<T>
}>

interface FocusHandler<T, S> {
    store(props: Readonly<{ data?: T; state: S }>): FocusState<T> | S
}

function initFocus<T, S>(
    props: FocusProps<T, S>,
): Readonly<{
    action: FocusAction<T, S> & { readonly state: ApplicationState<FocusState<T> | S> }
    handler: FocusHandler<T, S>
}> {
    const { state, post } = initApplicationState<FocusState<T> | S>({
        initialState: { type: "close" },
    })

    let element: PrepareElementState<T> = { isLoad: false }

    return {
        action: { state, onModify, change, update, remove, close, isFocused },
        handler: { store },
    }

    function onModify(handler: ModifyFieldHandler<T>): void {
        state.subscribe((state): true => {
            const modifyState = props.modifyState(state)
            if (modifyState.type === "noop") {
                return true
            }
            switch (modifyState.state.type) {
                case "change":
                    handler.focus(modifyState.state.data)
                    return true

                case "update":
                    handler.update(modifyState.state.data)
                    return true

                case "close":
                    handler.close()
                    return true
            }
        })
    }

    function store({ data, state }: Readonly<{ data?: T; state: S }>): FocusState<T> | S {
        if (data !== undefined) {
            element = { isLoad: true, data }
        }
        return post(state)
    }

    function change(data: T): FocusState<T> | S {
        if (!props.handler.find(data)) {
            return close()
        }

        element = { isLoad: true, data }
        return post({ type: "change", data })
    }
    function update(data: Partial<T>): FocusState<T> | S {
        if (!element.isLoad) {
            return state.currentState()
        }

        const newData = { ...element.data, ...data }
        props.handler.update(element.data, newData)

        element = { isLoad: true, data: newData }
        return post({ type: "update", data: newData })
    }
    function remove(): FocusState<T> | S {
        if (element.isLoad) {
            props.handler.remove(element.data)
        }
        return close()
    }
    function close(): FocusState<T> | S {
        element = { isLoad: false }
        return post({ type: "close" })
    }

    function isFocused(data: T): boolean {
        return element.isLoad && element.data === data
    }
}
