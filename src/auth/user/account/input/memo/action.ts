import { ApplicationAction } from "../../../../../z_vendor/getto-application/action/action"
import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../../z_vendor/getto-application/board/observe_field/action"
import { ValidateLoginIdAction } from "../../../login_id/input/action"

import { initBoardFieldObserver } from "../../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { authUserMemoBoardConverter } from "./convert"

import { BoardValueStore } from "../../../../../z_vendor/getto-application/board/input/infra"

import { AuthUserMemo } from "../../kernel/data"
import { ValidateAuthUserMemoError } from "./data"
import { ConvertBoardFieldResult } from "../../../../../z_vendor/getto-application/board/validate_field/data"
import { initValidateBoardFieldAction } from "../../../../../z_vendor/getto-application/board/validate_field/action"

export interface InputAuthUserMemoAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateLoginIdAction
    readonly observe: ObserveBoardFieldAction

    reset(memo: AuthUserMemo): void
}

export function initInputAuthUserMemoAction(): Readonly<{
    input: InputAuthUserMemoAction
    convert: { (): ConvertBoardFieldResult<AuthUserMemo, ValidateAuthUserMemoError> }
}> {
    const input = new InputAction()
    return {
        input,
        convert: () => input.convert(),
    }
}

class InputAction implements InputAuthUserMemoAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateLoginIdAction
    readonly observe: ObserveBoardFieldAction

    readonly store: Readonly<{
        memo: BoardValueStore
    }>

    terminate: { (): void }

    constructor() {
        const memo = initInputBoardAction()
        const { validate, checker } = initValidateBoardFieldAction({
            converter: () => this.convert(),
        })
        const observe = initObserveBoardFieldAction({
            observer: initBoardFieldObserver({
                current: () => memo.store.get(),
            }),
        })

        this.input = memo.input
        this.validate = validate
        this.observe = observe

        this.store = {
            memo: memo.store,
        }

        memo.subscriber.subscribe(() => {
            checker.check()
            observe.check()
        })

        this.terminate = () => {
            memo.subscriber.terminate()
            observe.terminate()
        }
    }

    convert(): ConvertBoardFieldResult<AuthUserMemo, ValidateAuthUserMemoError> {
        return authUserMemoBoardConverter(this.store.memo.get())
    }

    reset(memo: AuthUserMemo): void {
        this.store.memo.set(memo)
    }
}
