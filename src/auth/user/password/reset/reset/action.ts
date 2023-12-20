import { checkTakeLongtime } from "../../../../../common/util/timer/helper"

import { getScriptPath } from "../../../../sign/get_script_path/method"
import {
    startContinuousRenew,
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
    StartContinuousRenewEvent,
} from "../../../../ticket/authenticate/method"

import { Atom, initAtom, mapAtom } from "../../../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../../common/util/board/observe/action"
import { composeRegisterFieldBoard } from "../../../../../common/util/board/field/action"
import { PasswordField, initPasswordField } from "../../input/field/action"

import { WaitTime } from "../../../../../common/util/config/infra"
import { GetScriptPathConfig, GetScriptPathShell } from "../../../../sign/get_script_path/infra"
import { ResetPasswordRemote, ResetTokenDetecter } from "./infra"

import { LoadScriptError, ConvertScriptPathResult } from "../../../../sign/get_script_path/data"
import { ResetPasswordError, ResetPasswordFields } from "./data"
import { AuthTicket } from "../../../../ticket/kernel/data"
import { ConvertBoardResult } from "../../../../../common/util/board/kernel/data"
import { RepositoryError } from "../../../../../common/util/repository/data"
import { ConnectState } from "../../../../../common/util/connect/data"

export interface ResetPasswordAction {
    readonly state: Atom<ResetPasswordState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>

    readonly newPassword: PasswordField

    reset(): void
    submit(): Promise<ResetPasswordState>
    loadError(err: LoadScriptError): Promise<ResetPasswordState>
}

export type ResetPasswordState =
    | Readonly<{ type: "initial-reset" }>
    | ResetEvent
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

const initialState: ResetPasswordState = { type: "initial-reset" }

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
    takeLongtimeThreshold: WaitTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export function initResetPasswordAction(material: ResetPasswordMaterial): ResetPasswordAction {
    const resetPassword = initAtom({ initialState })
    async function registerWithCurrentState(): Promise<ResetPasswordState> {
        const fields = currentFields()
        if (!fields.valid) {
            return resetPassword.state.currentState()
        }
        const result = await resetAuthUserPassword(material, fields.value, resetPassword.post)
        if (!result.success) {
            return result.state
        }
        return startContinuousRenew(
            material,
            { hasTicket: true, ticket: result.ticket },
            (event) => {
                switch (event.type) {
                    case "succeed-to-start-continuous-renew":
                        return resetPassword.post({
                            type: "try-to-load",
                            scriptPath: getScriptPath(material),
                        })
                    default:
                        return resetPassword.post(event)
                }
            },
        )
    }
    async function loadError(err: LoadScriptError): Promise<ResetPasswordState> {
        return resetPassword.post({ type: "load-error", err })
    }

    const newPassword = initPasswordField()

    const currentFields = (): ConvertBoardResult<ResetPasswordFields> => {
        const result = {
            newPassword: newPassword[0].validate.currentState(),
        }
        if (!result.newPassword.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                newPassword: result.newPassword.value,
            },
        }
    }

    const { validate, observe, reset } = composeRegisterFieldBoard([newPassword])

    const connect = mapAtom(resetPassword.state, (state): ConnectState => {
        if (state.type === "try-to-reset") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: resetPassword.state,
        connect,
        validate,
        observe,

        newPassword: newPassword[0],

        reset,
        submit: registerWithCurrentState,
        loadError,
    }
}

type ResetEvent =
    | Readonly<{ type: "try-to-reset"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed-to-reset"; err: ResetPasswordError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type ResetResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function resetAuthUserPassword<S>(
    { infra, shell, config }: ResetPasswordMaterial,
    fields: ResetPasswordFields,
    post: Post<ResetEvent, S>,
): Promise<ResetResult<S>> {
    const resetToken = shell.detectResetToken()
    if (!resetToken.valid) {
        return {
            success: false,
            state: post({ type: "failed-to-reset", err: { type: "empty-reset-token" } }),
        }
    }

    post({ type: "try-to-reset", hasTakenLongtime: false })

    const { resetRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        resetRemote(resetToken.value, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try-to-reset", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return { success: false, state: post({ type: "failed-to-reset", err: response.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
