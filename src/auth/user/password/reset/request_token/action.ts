import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../../../ui/vendor/getto-application/action/action"
import { initSignLink } from "../../../../sign/nav/resource"
import { initInputLoginIDAction } from "../../../login_id/input/action"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/validate_board/action"

import { SignLink } from "../../../../sign/nav/resource"
import { InputLoginIDAction } from "../../../login_id/input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/validate_board/action"

import { RequestResetTokenRemote } from "./infra"
import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { ValidateBoardChecker } from "../../../../../../ui/vendor/getto-application/board/validate_board/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export interface RequestResetTokenAction extends StatefulApplicationAction<RequestResetTokenState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    clear(): void
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

export const initialRequestResetTokenState: RequestResetTokenState = {
    type: "initial-request-token",
}

export function initRequestResetTokenAction(
    material: RequestResetTokenMaterial,
): RequestResetTokenAction {
    return new Action(material)
}

const requestResetTokenFieldNames = ["loginID"] as const
type RequestResetTokenFieldName = typeof requestResetTokenFieldNames[number]

class Action
    extends AbstractStatefulApplicationAction<RequestResetTokenState>
    implements RequestResetTokenAction
{
    readonly initialState = initialRequestResetTokenState

    readonly link = initSignLink()

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    material: RequestResetTokenMaterial
    checker: ValidateBoardChecker<RequestResetTokenFieldName, RequestResetTokenFields>

    constructor(material: RequestResetTokenMaterial) {
        super({
            terminate: () => {
                this.loginID.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const loginID = initInputLoginIDAction()

        const { validate, checker } = initValidateBoardAction(
            {
                fields: requestResetTokenFieldNames,
            },
            {
                converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                    const loginIDResult = loginID.checker.check()
                    if (!loginIDResult.valid) {
                        return { valid: false }
                    }
                    return {
                        valid: true,
                        value: {
                            loginID: loginIDResult.value,
                        },
                    }
                },
            },
        )

        this.loginID = loginID.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )
    }

    clear(): void {
        this.loginID.clear()
        this.validate.clear()
    }
    submit(): Promise<RequestResetTokenState> {
        return requestResetToken(this.material, this.checker.get(), this.post)
    }
}

export interface RequestResetTokenProfileAction
    extends StatefulApplicationAction<RequestResetTokenProfileState> {
    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    open(): RequestResetTokenProfileState
    clear(): RequestResetTokenProfileState
    submit(): Promise<RequestResetTokenProfileState>
    close(): RequestResetTokenProfileState
}

export type RequestResetTokenProfileState =
    | Readonly<{ type: "initial-request-token" }>
    | Readonly<{ type: "input-login-id" }>
    | RequestResetTokenEvent

export const initialRequestResetTokenProfileState: RequestResetTokenProfileState = {
    type: "initial-request-token",
}

export function initRequestResetTokenProfileAction(
    material: RequestResetTokenMaterial,
): RequestResetTokenProfileAction {
    return new ProfileAction(material)
}

const requestResetTokenProfileFieldNames = ["loginID"] as const
type RequestResetTokenProfileFieldName = typeof requestResetTokenProfileFieldNames[number]

class ProfileAction
    extends AbstractStatefulApplicationAction<RequestResetTokenProfileState>
    implements RequestResetTokenProfileAction
{
    readonly initialState = initialRequestResetTokenProfileState

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    material: RequestResetTokenMaterial
    checker: ValidateBoardChecker<RequestResetTokenProfileFieldName, RequestResetTokenFields>

    constructor(material: RequestResetTokenMaterial) {
        super({
            terminate: () => {
                this.loginID.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const loginID = initInputLoginIDAction()

        const { validate, checker } = initValidateBoardAction(
            {
                fields: requestResetTokenProfileFieldNames,
            },
            {
                converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                    const loginIDResult = loginID.checker.check()
                    if (!loginIDResult.valid) {
                        return { valid: false }
                    }
                    return {
                        valid: true,
                        value: {
                            loginID: loginIDResult.value,
                        },
                    }
                },
            },
        )

        this.loginID = loginID.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )
    }

    open(): RequestResetTokenProfileState {
        this.clearInput()
        return this.post({ type: "input-login-id" })
    }
    clear(): RequestResetTokenProfileState {
        this.clearInput()
        return this.post({ type: "input-login-id" })
    }
    submit(): Promise<RequestResetTokenProfileState> {
        return requestResetToken(this.material, this.checker.get(), this.post)
    }
    close(): RequestResetTokenProfileState {
        this.clearInput()
        return this.post(this.initialState)
    }

    clearInput(): void {
        this.loginID.clear()
        this.validate.clear()
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
