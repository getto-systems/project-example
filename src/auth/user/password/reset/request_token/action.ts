import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/init"
import { initSignLink } from "../../../../sign/action_nav/init"
import { initInputLoginIDAction } from "../../../login_id/input/action"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import { SignLink } from "../../../../sign/action_nav/resource"
import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputLoginIDAction } from "../../../login_id/input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { RequestResetTokenRemote } from "./infra"
import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { ValidateBoardChecker } from "../../../../../../ui/vendor/getto-application/board/validate_board/infra"

import { RequestResetTokenError, RequestResetTokenFields } from "./data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export interface RequestResetTokenAction extends ApplicationStateAction<RequestResetTokenState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    clear(): void
    submit(): Promise<RequestResetTokenState>
}

export type RequestResetTokenState =
    | Readonly<{ type: "initial-request-token" }>
    | RequestResetTokenEvent

export type RequestResetTokenConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export type RequestResetTokenInfra = Readonly<{
    requestTokenRemote: RequestResetTokenRemote
}>

export const initialRequestResetTokenState: RequestResetTokenState = {
    type: "initial-request-token",
}

export function initRequestResetTokenAction(
    config: RequestResetTokenConfig,
    infra: RequestResetTokenInfra,
): RequestResetTokenAction {
    return new Action(config, infra)
}

const requestResetTokenFieldNames = ["loginID"] as const
type RequestResetTokenFieldName = typeof requestResetTokenFieldNames[number]

class Action
    extends ApplicationAbstractStateAction<RequestResetTokenState>
    implements RequestResetTokenAction
{
    readonly initialState = initialRequestResetTokenState

    readonly link = initSignLink()

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    config: RequestResetTokenConfig
    infra: RequestResetTokenInfra
    checker: ValidateBoardChecker<RequestResetTokenFieldName, RequestResetTokenFields>

    constructor(config: RequestResetTokenConfig, infra: RequestResetTokenInfra) {
        super()
        this.config = config
        this.infra = infra

        const loginID = initInputLoginIDAction()

        const { validate, checker } = initValidateBoardAction({
            fields: requestResetTokenFieldNames,
            converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                const loginIDResult = loginID.checker.get()
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
        })

        this.loginID = loginID.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.validate.terminate()
        })
    }

    clear(): void {
        this.loginID.clear()
        this.validate.clear()
    }
    submit(): Promise<RequestResetTokenState> {
        return requestResetToken(this.config, this.infra, this.checker.get(), this.post)
    }
}

export interface RequestResetTokenProfileAction
    extends ApplicationStateAction<RequestResetTokenProfileState> {
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
    config: RequestResetTokenConfig,
    infra: RequestResetTokenInfra,
): RequestResetTokenProfileAction {
    return new ProfileAction(config, infra)
}

const requestResetTokenProfileFieldNames = ["loginID"] as const
type RequestResetTokenProfileFieldName = typeof requestResetTokenProfileFieldNames[number]

class ProfileAction
    extends ApplicationAbstractStateAction<RequestResetTokenProfileState>
    implements RequestResetTokenProfileAction
{
    readonly initialState = initialRequestResetTokenProfileState

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    config: RequestResetTokenConfig
    infra: RequestResetTokenInfra
    checker: ValidateBoardChecker<RequestResetTokenProfileFieldName, RequestResetTokenFields>

    constructor(config: RequestResetTokenConfig, infra: RequestResetTokenInfra) {
        super()
        this.config = config
        this.infra = infra

        const loginID = initInputLoginIDAction()

        const { validate, checker } = initValidateBoardAction({
            fields: requestResetTokenProfileFieldNames,
            converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                const loginIDResult = loginID.checker.get()
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
        })

        this.loginID = loginID.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.validate.terminate()
        })
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
        return requestResetToken(this.config, this.infra, this.checker.get(), this.post)
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
    config: RequestResetTokenConfig,
    infra: RequestResetTokenInfra,
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
