import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/validate_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"
import { ValidateTextError } from "../../validate/data"

export interface TextFieldAction<T> {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<T, readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction

    clear(): void
    reset(value: T): void
}

export type TextFieldProps<T> = Readonly<{
    convert: (value: string) => ValidateBoardFieldResult<T, readonly ValidateTextError[]>
}>
export function initTextFieldAction<T extends string>(
    props: TextFieldProps<T>,
): TextFieldAction<T> {
    const { input } = initAction(props)
    return input
}

export type TextFieldPropsWithResource<T, R> = TextFieldProps<T> &
    Readonly<{
        resource: (
            props: Readonly<{
                store: BoardValueStore
                subscriber: TextFieldActionSubscriber
            }>,
        ) => R
    }>
export function initTextFieldActionWithResource<T extends string, R>(
    props: TextFieldPropsWithResource<T, R>,
): TextFieldAction<T> & R {
    const { input, store, subscriber } = initAction(props)
    return {
        ...input,
        ...props.resource({ store, subscriber }),
    }
}

function initAction<T extends string>(
    props: TextFieldProps<T>,
): Readonly<{
    input: TextFieldAction<T>
    store: BoardValueStore
    subscriber: TextFieldActionSubscriber
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        convert: () => props.convert(store.get()),
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    const [actionSubscriber, post] = initActionSubscriber()

    subscriber.subscribe(() => {
        validate.check()
        observe.check()
        post.onInput()
    })

    return {
        input: {
            input,
            validate,
            observe,

            clear: () => {
                store.set("")
                post.onClear()
            },
            reset: (value) => {
                store.set(value)
                post.onReset()
            },
        },
        store,
        subscriber: actionSubscriber,
    }
}

export interface TextFieldActionSubscriber {
    subscribe(handler: TextFieldActionHandler): void
}
export type TextFieldActionHandler = Readonly<{
    onInput?: () => void
    onClear?: () => void
    onReset?: () => void
}>

function initActionSubscriber(): [
    TextFieldActionSubscriber,
    {
        onInput(): void
        onClear(): void
        onReset(): void
    },
] {
    let handlers: TextFieldActionHandler[] = []

    return [
        {
            subscribe: (handler) => {
                handlers = [handler, ...handlers]
            },
        },
        {
            onInput: () => {
                handlers.forEach((handler) => handler.onInput?.())
            },
            onClear: () => {
                handlers.forEach((handler) => handler.onClear?.())
            },
            onReset: () => {
                handlers.forEach((handler) => handler.onReset?.())
            },
        },
    ]
}
