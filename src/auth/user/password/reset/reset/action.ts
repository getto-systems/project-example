import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../../../ui/vendor/getto-application/action/action"

import { initSignLink } from "../../../../sign/nav/resource"
import { initInputLoginIDAction } from "../../../login_id/input/action"
import { initInputPasswordAction } from "../../input/action"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/validate_board/action"

import { SignLink } from "../../../../sign/nav/resource"
import { InputLoginIDAction } from "../../../login_id/input/action"
import { InputPasswordAction } from "../../input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/validate_board/action"

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

export interface ResetPasswordAction extends StatefulApplicationAction<ResetPasswordState> {
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

export type ResetPasswordMaterial = Readonly<{
    infra: ResetPasswordInfra
    shell: ResetPasswordShell
    config: ResetPasswordConfig
}>

export type ResetPasswordInfra = Readonly<{
    resetRemote: ResetPasswordRemote
}> &
    StartContinuousRenewInfra

export type ResetPasswordShell = Readonly<{
    detectResetToken: ResetTokenDetecter
}> &
    GetScriptPathShell

export type ResetPasswordConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export function initResetPasswordAction(material: ResetPasswordMaterial): ResetPasswordAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<ResetPasswordState>
    implements ResetPasswordAction
{
    readonly initialState = initialResetPasswordState

    readonly link = initSignLink()

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    material: ResetPasswordMaterial
    checker: ValidateBoardChecker<ResetPasswordFieldName, ResetPasswordFields>

    constructor(material: ResetPasswordMaterial) {
        super({
            terminate: () => {
                this.loginID.terminate()
                this.password.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const loginID = initInputLoginIDAction()
        const password = initInputPasswordAction()

        const { validate, checker } = initValidateBoardAction(
            {
                fields: resetPasswordFieldNames,
            },
            {
                converter: (): ConvertBoardResult<ResetPasswordFields> => {
                    const loginIDResult = loginID.checker.check()
                    const passwordResult = password.checker.check()
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
            },
        )

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
    }

    clear(): void {
        this.loginID.clear()
        this.password.clear()
        this.validate.clear()
    }
    async submit(): Promise<ResetPasswordState> {
        const result = await reset(this.material, this.checker.get(), this.post)
        if (!result.success) {
            return result.state
        }
        return this.startContinuousRenew(result.ticket)
    }
    async startContinuousRenew(ticket: AuthTicket): Promise<ResetPasswordState> {
        return await startContinuousRenew(this.material, { hasTicket: true, ticket }, (event) => {
            switch (event.type) {
                case "succeed-to-start-continuous-renew":
                    return this.post({
                        type: "try-to-load",
                        scriptPath: this.secureScriptPath(),
                    })
                default:
                    return this.post(event)
            }
        })
    }

    async loadError(err: LoadScriptError): Promise<ResetPasswordState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return getScriptPath(this.material)
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
    { infra, shell, config }: ResetPasswordMaterial,
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
