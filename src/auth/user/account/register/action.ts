import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"

import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import {
    initResetTokenDestinationFieldAction,
    ResetTokenDestinationFieldAction,
} from "../../password/reset/token_destination/input/action"
import {
    AuthPermissionGrantedFieldAction,
    AuthUserTextFieldAction,
    initAuthPermissionGrantedFieldAction,
    initAuthUserTextFieldAction,
} from "../input/field/action"
import { initLoginIdFieldAction, LoginIdFieldAction } from "../../login_id/input/action"
import { initRegisterField } from "../../../../common/util/register/action"
import { initListRegisteredAction, ListRegisteredAction } from "../../../../common/util/list/action"

import { ALL_AUTH_PERMISSIONS } from "../../../../x_content/permission"

import { RegisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { RegisterAuthUserAccountError } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { AuthUserAccount } from "../kernel/data"

export interface RegisterAuthUserAccountAction {
    readonly state: ApplicationState<RegisterAuthUserAccountState>
    readonly list: ListRegisteredAction<AuthUserAccount>

    readonly loginId: LoginIdFieldAction
    readonly granted: AuthPermissionGrantedFieldAction
    readonly resetTokenDestination: ResetTokenDestinationFieldAction
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): void
    submit(): Promise<RegisterAuthUserAccountState>
}

export type RegisterAuthUserAccountState = RegisterUserEvent

const initialState: RegisterAuthUserAccountState = { type: "initial" }

export type RegisterAuthUserAccountMaterial = Readonly<{
    infra: RegisterAuthUserAccountInfra
    config: RegisterAuthUserAccountConfig
}>

export type RegisterAuthUserAccountInfra = Readonly<{
    registerUserRemote: RegisterAuthUserAccountRemote
}>

export type RegisterAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initRegisterAuthUserAccountAction(
    material: RegisterAuthUserAccountMaterial,
): RegisterAuthUserAccountAction {
    const { state, post } = initApplicationState({ initialState })

    const loginId = initLoginIdFieldAction()
    const granted = initAuthPermissionGrantedFieldAction()
    const resetTokenDestination = initResetTokenDestinationFieldAction()
    const memo = initAuthUserTextFieldAction("memo")

    const convert = (): ConvertBoardResult<AuthUserAccount> => {
        const result = {
            loginId: loginId.validate.check(),
            granted: granted.input.validate.check(),
            resetTokenDestination: resetTokenDestination.validate.check(),
            memo: memo.validate.check(),
        }
        if (
            !result.loginId.valid ||
            !result.granted.valid ||
            !result.resetTokenDestination.valid ||
            !result.memo.valid
        ) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginId: result.loginId.value,
                granted: result.granted.value,
                resetTokenDestination: result.resetTokenDestination.value,
                memo: result.memo.value,
            },
        }
    }

    const { validate, observe, clear } = initRegisterField(
        [
            ["loginId", loginId],
            ["granted", granted.input],
            ["resetTokenDestination", resetTokenDestination],
            ["memo", memo],
        ],
        convert,
    )

    granted.setOptions(ALL_AUTH_PERMISSIONS)

    const list = initListRegisteredAction<AuthUserAccount>()

    clear()

    onSuccess((data) => {
        clear()
        list.handler.register(data)
    })

    return {
        state,

        list: list.action,

        loginId,
        granted: granted.input,
        resetTokenDestination,
        memo,

        validate,
        observe,
        clear,

        async submit(): Promise<RegisterAuthUserAccountState> {
            const fields = convert()
            if (!fields.valid) {
                return state.currentState()
            }
            return registerUser(material, fields.value, post)
        },
    }

    function onSuccess(handler: (data: AuthUserAccount) => void): void {
        state.subscribe((state) => {
            switch (state.type) {
                case "success":
                    handler(state.data)
                    break
            }
        })
    }
}

type RegisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RegisterAuthUserAccountError }>
    | Readonly<{ type: "success"; data: AuthUserAccount }>
    | Readonly<{ type: "initial" }>

async function registerUser<S>(
    { infra, config }: RegisterAuthUserAccountMaterial,
    fields: AuthUserAccount,
    post: Post<RegisterUserEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { registerUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        registerUserRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: fields })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
