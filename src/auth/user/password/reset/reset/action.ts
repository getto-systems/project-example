import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/init"
import { initSignLink } from "../../../../sign/nav/resource"
import { initInputLoginIDAction } from "../../../login_id/input/action"
import { initInputPasswordAction } from "../../input/action"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import { SignLink } from "../../../../sign/nav/resource"
import { InputLoginIDAction } from "../../../login_id/input/action"
import { InputPasswordAction } from "../../input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import { getScriptPath } from "../../../../sign/get_script_path/method"
import { startContinuousRenew } from "../../../../ticket/start_continuous_renew/method"

import {
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
} from "../../../../ticket/start_continuous_renew/infra"
import { GetScriptPathConfig, GetScriptPathShell } from "../../../../sign/get_script_path/infra"
import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { ResetPasswordRemote, ResetTokenDetecter } from "./infra"
import { ValidateBoardChecker } from "../../../../../../ui/vendor/getto-application/board/validate_board/infra"

import { StartContinuousRenewEvent } from "../../../../ticket/start_continuous_renew/event"

import { LoadScriptError, ConvertScriptPathResult } from "../../../../sign/get_script_path/data"
import { ResetPasswordError, ResetPasswordFields } from "./data"
import { AuthTicket } from "../../../../ticket/kernel/data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RepositoryError } from "../../../../../z_lib/ui/repository/data"

export interface ResetPasswordAction extends ApplicationStateAction<ResetPasswordState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): void
    submit(): Promise<ResetPasswordState>
    loadError(err: LoadScriptError): Promise<ResetPasswordState>
}

export const resetPasswordFieldNames = ["loginID", "password"] as const
export type ResetPasswordFieldName = typeof resetPasswordFieldNames[number]

export type ResetPasswordState =
    | Readonly<{ type: "initial-reset" }>
    | ResetEvent
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialResetPasswordState: ResetPasswordState = {
    type: "initial-reset",
}

export type ResetPasswordConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export type ResetPasswordInfra = Readonly<{
    resetRemote: ResetPasswordRemote
}> &
    StartContinuousRenewInfra

export type ResetPasswordShell = Readonly<{
    detectResetToken: ResetTokenDetecter
}> &
    GetScriptPathShell

export function initResetPasswordAction(
    config: ResetPasswordConfig,
    infra: ResetPasswordInfra,
    shell: ResetPasswordShell,
): ResetPasswordAction {
    return new Action(config, infra, shell)
}

class Action
    extends ApplicationAbstractStateAction<ResetPasswordState>
    implements ResetPasswordAction
{
    readonly initialState = initialResetPasswordState

    readonly link = initSignLink()

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    config: ResetPasswordConfig
    infra: ResetPasswordInfra
    shell: ResetPasswordShell
    checker: ValidateBoardChecker<ResetPasswordFieldName, ResetPasswordFields>

    constructor(config: ResetPasswordConfig, infra: ResetPasswordInfra, shell: ResetPasswordShell) {
        super()
        this.config = config
        this.infra = infra
        this.shell = shell

        const loginID = initInputLoginIDAction()
        const password = initInputPasswordAction()

        const { validate, checker } = initValidateBoardAction({
            fields: resetPasswordFieldNames,
            converter: (): ConvertBoardResult<ResetPasswordFields> => {
                const loginIDResult = loginID.checker.get()
                const passwordResult = password.checker.get()
                if (!loginIDResult.valid || !passwordResult.valid) {
                    return { valid: false }
                }
                return {
                    valid: true,
                    value: {
                        loginID: loginIDResult.value,
                        password: passwordResult.value,
                    },
                }
            },
        })

        this.loginID = loginID.input
        this.password = password.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )
        this.password.validate.subscriber.subscribe((result) =>
            checker.update("password", result.valid),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.password.terminate()
            this.validate.terminate()
        })
    }

    clear(): void {
        this.loginID.clear()
        this.password.clear()
        this.validate.clear()
    }
    async submit(): Promise<ResetPasswordState> {
        const result = await reset(
            this.config,
            this.infra,
            this.shell,
            this.checker.get(),
            this.post,
        )
        if (!result.success) {
            return result.state
        }
        return this.startContinuousRenew(result.ticket)
    }
    async startContinuousRenew(ticket: AuthTicket): Promise<ResetPasswordState> {
        return await startContinuousRenew(
            this.config,
            this.infra,
            { hold: true, ticket },
            (event) => {
                switch (event.type) {
                    case "succeed-to-start-continuous-renew":
                        return this.post({
                            type: "try-to-load",
                            scriptPath: this.secureScriptPath(),
                        })
                    default:
                        return this.post(event)
                }
            },
        )
    }

    async loadError(err: LoadScriptError): Promise<ResetPasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return getScriptPath(this.config, this.shell)
    }
}

type ResetEvent =
    | Readonly<{ type: "try-to-reset" }>
    | Readonly<{ type: "take-longtime-to-reset" }>
    | Readonly<{ type: "failed-to-reset"; err: ResetPasswordError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type ResetResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function reset<S>(
    config: ResetPasswordConfig,
    infra: ResetPasswordInfra,
    shell: ResetPasswordShell,
    fields: ConvertBoardResult<ResetPasswordFields>,
    post: Post<ResetEvent, S>,
): Promise<ResetResult<S>> {
    if (!fields.valid) {
        return {
            success: false,
            state: post({ type: "failed-to-reset", err: { type: "validation-error" } }),
        }
    }

    const resetToken = shell.detectResetToken()
    if (!resetToken.valid) {
        return {
            success: false,
            state: post({ type: "failed-to-reset", err: { type: "empty-reset-token" } }),
        }
    }

    post({ type: "try-to-reset" })

    const { resetRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        resetRemote(resetToken.value, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-reset" }),
    )
    if (!response.success) {
        return { success: false, state: post({ type: "failed-to-reset", err: response.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
