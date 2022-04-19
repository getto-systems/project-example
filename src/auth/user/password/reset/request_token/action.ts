import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../../z_vendor/getto-application/action/action"

import { InputLoginIdAction, initInputLoginIdAction } from "../../../login_id/input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../../z_vendor/getto-application/board/observe_board/action"

import { RequestResetTokenRemote } from "./infra"
import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { BoardConverter } from "../../../../../z_vendor/getto-application/board/kernel/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../z_vendor/getto-application/board/kernel/data"

export interface RequestResetTokenAction extends StatefulApplicationAction<RequestResetTokenState> {
    readonly loginId: InputLoginIdAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): RequestResetTokenState
    submit(): Promise<RequestResetTokenState>
}

export type RequestResetTokenState = Readonly<{ type: "initial" }> | RequestResetTokenEvent

export type RequestResetTokenMaterial = Readonly<{
    infra: RequestResetTokenInfra
    config: RequestResetTokenConfig
}>

export type RequestResetTokenInfra = Readonly<{
    requestTokenRemote: RequestResetTokenRemote
}>

export type RequestResetTokenConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

const initialState: RequestResetTokenState = { type: "initial" }

export function initRequestResetTokenAction(
    material: RequestResetTokenMaterial,
): RequestResetTokenAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<RequestResetTokenState>
    implements RequestResetTokenAction
{
    readonly initialState = initialState

    readonly loginId: InputLoginIdAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: RequestResetTokenMaterial
    convert: BoardConverter<RequestResetTokenFields>

    constructor(material: RequestResetTokenMaterial) {
        super({
            terminate: () => {
                this.loginId.terminate()
                this.validate.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const fields = ["loginId"] as const

        const loginId = initInputLoginIdAction()

        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                    const loginIdResult = loginId.checker.check()
                    if (!loginIdResult.valid) {
                        return { valid: false }
                    }
                    return {
                        valid: true,
                        value: {
                            loginId: loginIdResult.value,
                        },
                    }
                },
            },
        )
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.loginId = loginId.input
        this.validate = validate
        this.observe = observe
        this.convert = () => validateChecker.get()

        this.loginId.validate.subscriber.subscribe((result) =>
            validateChecker.update("loginId", result.valid),
        )
        this.loginId.observe.subscriber.subscribe((result) =>
            observeChecker.update("loginId", result.hasChanged),
        )
    }

    clear(): RequestResetTokenState {
        this.loginId.clear()
        this.validate.clear()
        return this.currentState()
    }
    async submit(): Promise<RequestResetTokenState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return requestResetToken(this.material, fields.value, this.post)
    }
}

type RequestResetTokenEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: RequestResetTokenError }>
    | Readonly<{ type: "success" }>

async function requestResetToken<S>(
    { infra, config }: RequestResetTokenMaterial,
    fields: RequestResetTokenFields,
    post: Post<RequestResetTokenEvent, S>,
): Promise<S> {
    post({ type: "try" })

    const { requestTokenRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        requestTokenRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
