import { checkTakeLongtime } from "../../../../common/util/timer/helper"

import { getScriptPath } from "../../../sign/get_script_path/method"
import {
    startContinuousRenew,
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
    StartContinuousRenewEvent,
} from "../../../ticket/authenticate/method"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../common/util/board/observe/action"
import { LoginIdField, initLoginIdField } from "../../login_id/input/field/action"
import { composeRegisterFieldBoard } from "../../../../common/util/board/field/action"
import { PasswordField, initPasswordField } from "../input/field/action"

import { WaitTime } from "../../../../common/util/config/infra"
import { GetScriptPathConfig, GetScriptPathShell } from "../../../sign/get_script_path/infra"
import { AuthenticatePasswordRemote } from "./infra"

import { LoadScriptError, ConvertScriptPathResult } from "../../../sign/get_script_path/data"
import { AuthenticatePasswordError, AuthenticatePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../common/util/board/kernel/data"
import { AuthTicket } from "../../../ticket/kernel/data"
import { RepositoryError } from "../../../../common/util/repository/data"
import { ConnectState } from "../../../../common/util/connect/data"

export interface AuthenticatePasswordAction {
    readonly state: Atom<AuthenticatePasswordState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>

    readonly loginId: LoginIdField
    readonly password: PasswordField

    reset(): void
    submit(): Promise<AuthenticatePasswordState>
    loadError(err: LoadScriptError): Promise<AuthenticatePasswordState>
}

export type AuthenticatePasswordState =
    | Readonly<{ type: "initial-login" }>
    | AuthenticateEvent
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

const initialState: AuthenticatePasswordState = { type: "initial-login" }

export type AuthenticatePasswordMaterial = Readonly<{
    infra: AuthenticatePasswordInfra
    shell: AuthenticatePasswordShell
    config: AuthenticatePasswordConfig
}>

export type AuthenticatePasswordInfra = Readonly<{
    authenticateRemote: AuthenticatePasswordRemote
}> &
    StartContinuousRenewInfra

export type AuthenticatePasswordShell = GetScriptPathShell

export type AuthenticatePasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export function initAuthenticatePasswordAction(
    material: AuthenticatePasswordMaterial,
): AuthenticatePasswordAction {
    const authenticate = initAtom({ initialState })
    async function registerWithCurrentState(): Promise<AuthenticatePasswordState> {
        const fields = currentFields()
        if (!fields.valid) {
            return authenticate.state.currentState()
        }
        const result = await authenticateWithPassword(material, fields.value, authenticate.post)
        if (!result.success) {
            return result.state
        }
        return startContinuousRenew(
            material,
            { hasTicket: true, ticket: result.ticket },
            (event) => {
                switch (event.type) {
                    case "succeed-to-start-continuous-renew":
                        return authenticate.post({
                            type: "try-to-load",
                            scriptPath: getScriptPath(material),
                        })
                    default:
                        return authenticate.post(event)
                }
            },
        )
    }
    async function loadError(err: LoadScriptError): Promise<AuthenticatePasswordState> {
        return authenticate.post({ type: "load-error", err })
    }

    const loginId = initLoginIdField()
    const password = initPasswordField()

    const currentFields = (): ConvertBoardResult<AuthenticatePasswordFields> => {
        const result = {
            loginId: loginId[0].validate.currentState(),
            password: password[0].validate.currentState(),
        }
        if (!result.loginId.valid || !result.password.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginId: result.loginId.value,
                password: result.password.value,
            },
        }
    }

    const { validate, observe, reset } = composeRegisterFieldBoard([loginId, password])

    const connect = mapAtom(authenticate.state, (state): ConnectState => {
        if (state.type === "try-to-login") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: authenticate.state,
        connect,
        validate,
        observe,

        loginId: loginId[0],
        password: password[0],

        reset,
        submit: registerWithCurrentState,
        loadError,
    }
}

type AuthenticateMaterial = Readonly<{
    infra: AuthenticatePasswordInfra
    config: AuthenticatePasswordConfig
}>

type AuthenticateEvent =
    | Readonly<{ type: "try-to-login"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed-to-login"; err: AuthenticatePasswordError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type AuthenticateResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function authenticateWithPassword<S>(
    { infra, config }: AuthenticateMaterial,
    fields: AuthenticatePasswordFields,
    post: Post<AuthenticateEvent, S>,
): Promise<AuthenticateResult<S>> {
    post({ type: "try-to-login", hasTakenLongtime: false })

    const { authenticateRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        authenticateRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try-to-login", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return { success: false, state: post({ type: "failed-to-login", err: response.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
