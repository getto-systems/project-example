import { ApplicationAction } from "../../../../../z_vendor/getto-application/action/action"
import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { authUserMemoBoardConverter } from "./convert"

import { BoardValueStore } from "../../../../../z_vendor/getto-application/board/input/infra"

import { AuthUserMemo } from "../../kernel/data"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../../../z_vendor/getto-application/board/validate_field/action"
import { ConvertAuthUserMemoResult } from "./data"
import { ValidateTextError } from "../../../../../z_lib/ui/validate/data"

export interface InputAuthUserMemoAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction

    reset(memo: AuthUserMemo): void
}

export function initInputAuthUserMemoAction(): Readonly<{
    input: InputAuthUserMemoAction
    convert: { (): ConvertAuthUserMemoResult }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const convert = () => authUserMemoBoardConverter(store.get())

    const { validate, checker } = initValidateBoardFieldAction({
        converter: convert,
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    subscriber.subscribe(() => {
        checker.check()
        observe.check()
    })

    return {
        input: {
            terminate: () => {
                subscriber.terminate()
                validate.terminate()
                observe.terminate()
            },

            input,
            validate,
            observe,

            reset: (value: AuthUserMemo) => {
                store.set(value)
            },
        },
        convert,
    }
}
