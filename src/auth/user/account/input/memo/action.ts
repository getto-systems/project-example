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
import { ValidateTextError } from "../../../../../z_lib/ui/validate/data"

export interface InputAuthUserMemoAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<AuthUserMemo, readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction

    reset(memo: AuthUserMemo): void
}

export function initInputAuthUserMemoAction(): InputAuthUserMemoAction {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        convert: () => authUserMemoBoardConverter(store.get()),
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    subscriber.subscribe(() => {
        validate.check()
        observe.check()
    })

    return {
        input,
        validate,
        observe,

        reset: (value: AuthUserMemo) => {
            store.set(value)
            observe.pin()
        },
    }
}
