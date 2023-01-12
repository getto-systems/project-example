import {
    initMultipleInputBoardAction,
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
import { initTextFieldActionSubscriber, TextFieldActionSubscriber } from "./init/pubsub"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"
import { ValidateTextError } from "../../validate/data"

export type TextVectorFieldAction<T> = VectorFieldAction<T, ValidateTextError>

export type TextVectorFieldProps<T> = Readonly<{
    convert: (value: string) => ValidateBoardFieldResult<T, readonly ValidateTextError[]>
}>
export function initTextVectorFieldAction<T extends string>(
    props: TextVectorFieldProps<T>,
): TextVectorFieldAction<T> {
    return initVectorFieldAction({ map: (value) => value, ...props })
}

export interface VectorFieldAction<T, E> {
    readonly input: InputBoardAction<MultipleBoardValueStore>
    readonly validate: ValidateBoardFieldAction<readonly T[], readonly E[]>
    readonly observe: ObserveBoardFieldAction

    clear(): void
    reset(value: readonly T[]): void
}

export type VectorFieldProps<T, E> = Readonly<{
    map: (value: T) => string
    convert: (value: string) => ValidateBoardFieldResult<T, readonly E[]>
}>
export function initVectorFieldAction<T, E>(
    props: VectorFieldProps<T, E>,
): VectorFieldAction<T, E> {
    const { input } = initAction(props)
    return input
    /* c8 ignore next */
}

function initAction<T, E>(
    props: VectorFieldProps<T, E>,
): Readonly<{
    input: VectorFieldAction<T, E>
    store: MultipleBoardValueStore
    subscriber: TextFieldActionSubscriber
}> {
    const { input, store, subscriber } = initMultipleInputBoardAction()

    const validate = initValidateBoardFieldAction({
        convert: () => {
            const acc = store.get().reduce(
                (acc, value) => {
                    const result = props.convert(value)
                    if (result.valid) {
                        acc.values.push(result.value)
                    } else {
                        acc.errors.push(...result.err)
                    }
                    return acc
                },
                <Readonly<{ values: T[]; errors: E[] }>>{
                    values: [],
                    errors: [],
                },
            )
            if (acc.errors.length > 0) {
                return { valid: false, err: acc.errors }
            } else {
                return { valid: true, value: acc.values }
            }
        },
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    const [actionSubscriber, post] = initTextFieldActionSubscriber()

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
                store.set([])
                post.onClear()
            },
            reset: (value) => {
                store.set(value.map(props.map))
                post.onReset()
            },
        },
        store,
        subscriber: actionSubscriber,
    }
}
