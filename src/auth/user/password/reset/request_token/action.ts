import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../../z_vendor/getto-application/action/action"
import { initInputLoginIdAction } from "../../../login_id/input/action"
import { initValidateBoardAction } from "../../../../../z_vendor/getto-application/board/validate_board/action"

import { InputLoginIdAction } from "../../../login_id/input/action"
import { ValidateBoardAction } from "../../../../../z_vendor/getto-application/board/validate_board/action"

import { RequestResetTokenRemote } from "./infra"
import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { ValidateBoardChecker } from "../../../../../z_vendor/getto-application/board/validate_board/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../z_vendor/getto-application/board/kernel/data"

export interface RequestResetTokenAction extends StatefulApplicationAction<RequestResetTokenState> {
    readonly loginId: InputLoginIdAction
    readonly validate: ValidateBoardAction

    clear(): RequestResetTokenState
    submit(): Promise<RequestResetTokenState>
}

export type RequestResetTokenState =
    | Readonly<{ type: "initial-request-token" }>
    | RequestResetTokenEvent

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

const initialState: RequestResetTokenState = { type: "initial-request-token" }

export function initRequestResetTokenAction(
    material: RequestResetTokenMaterial,
): RequestResetTokenAction {
    return new Action(material)
}

const requestResetTokenFieldNames = ["loginId"] as const
type RequestResetTokenFieldName = typeof requestResetTokenFieldNames[number]

class Action
    extends AbstractStatefulApplicationAction<RequestResetTokenState>
    implements RequestResetTokenAction
{
    readonly initialState = initialState

    readonly loginId: InputLoginIdAction
    readonly validate: ValidateBoardAction

    material: RequestResetTokenMaterial
    checker: ValidateBoardChecker<RequestResetTokenFieldName, RequestResetTokenFields>

    constructor(material: RequestResetTokenMaterial) {
        super({
            terminate: () => {
                this.loginId.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const loginId = initInputLoginIdAction()

        const { validate, checker } = initValidateBoardAction(
            {
                fields: requestResetTokenFieldNames,
            },
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

        this.loginId = loginId.input
        this.validate = validate
        this.checker = checker

        this.loginId.validate.subscriber.subscribe((result) =>
            checker.update("loginId", result.valid),
        )
    }

    clear(): RequestResetTokenState {
        this.loginId.clear()
        this.validate.clear()
        return this.currentState()
    }
    submit(): Promise<RequestResetTokenState> {
        return requestResetToken(this.material, this.checker.get(), this.post)
    }
}

type RequestResetTokenEvent =
    | Readonly<{ type: "try-to-request-token" }>
    | Readonly<{ type: "take-longtime-to-request-token" }>
    | Readonly<{ type: "failed-to-request-token"; err: RequestResetTokenError }>
    | Readonly<{ type: "succeed-to-request-token" }>

async function requestResetToken<S>(
    { infra, config }: RequestResetTokenMaterial,
    fields: ConvertBoardResult<RequestResetTokenFields>,
    post: Post<RequestResetTokenEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed-to-request-token", err: { type: "validation-error" } })
    }

    post({ type: "try-to-request-token" })

    const { requestTokenRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        requestTokenRemote(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-request-token" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-request-token", err: response.err })
    }

    return post({ type: "succeed-to-request-token" })
}

interface Post<E, S> {
    (event: E): S
}
