import {
    ApplicationState,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import {
    initResetTokenDestinationFieldAction,
    ResetTokenDestinationFieldAction,
} from "../../password/reset/token_destination/input/action"
import {
    AuthUserGrantedRolesFieldAction,
    AuthUserTextFieldAction,
    initAuthUserGrantedRolesFieldAction,
    initAuthUserTextFieldAction,
} from "../input/field/action"
import { initLoginIdFieldAction, LoginIdFieldAction } from "../../login_id/input/action"
import { initRegisterField } from "../../../../z_lib/ui/register/action"
import { initListRegisteredAction, ListRegisteredAction } from "../../../../z_lib/ui/list/action"

import { ALL_AUTH_ROLES } from "../../../../x_content/role"

import { RegisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { RegisterAuthUserAccountError } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { AuthUserAccount } from "../kernel/data"

export interface RegisterAuthUserAccountAction
    extends StatefulApplicationAction<RegisterAuthUserAccountState> {
    readonly list: ListRegisteredAction<AuthUserAccount>

    readonly loginId: LoginIdFieldAction
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
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
    return new Action(material)
}

class Action implements RegisterAuthUserAccountAction {
    readonly material: RegisterAuthUserAccountMaterial
    readonly state: ApplicationState<RegisterAuthUserAccountState>
    readonly post: (state: RegisterAuthUserAccountState) => RegisterAuthUserAccountState

    readonly list: ListRegisteredAction<AuthUserAccount>

    readonly loginId: LoginIdFieldAction
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
    readonly resetTokenDestination: ResetTokenDestinationFieldAction
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: () => ConvertBoardResult<AuthUserAccount>
    clear: () => void

    constructor(material: RegisterAuthUserAccountMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

        const loginId = initLoginIdFieldAction()
        const grantedRoles = initAuthUserGrantedRolesFieldAction()
        const resetTokenDestination = initResetTokenDestinationFieldAction()
        const memo = initAuthUserTextFieldAction("memo")

        const convert = (): ConvertBoardResult<AuthUserAccount> => {
            const result = {
                loginId: loginId.validate.check(),
                grantedRoles: grantedRoles.input.validate.check(),
                resetTokenDestination: resetTokenDestination.validate.check(),
                memo: memo.validate.check(),
            }
            if (
                !result.loginId.valid ||
                !result.grantedRoles.valid ||
                !result.resetTokenDestination.valid ||
                !result.memo.valid
            ) {
                return { valid: false }
            }
            return {
                valid: true,
                value: {
                    loginId: result.loginId.value,
                    grantedRoles: result.grantedRoles.value,
                    resetTokenDestination: result.resetTokenDestination.value,
                    memo: result.memo.value,
                },
            }
        }

        const { validate, observe, clear } = initRegisterField(
            [
                ["loginId", loginId],
                ["grantedRoles", grantedRoles.input],
                ["resetTokenDestination", resetTokenDestination],
                ["memo", memo],
            ],
            convert,
        )

        grantedRoles.setOptions(ALL_AUTH_ROLES)

        const list = initListRegisteredAction<AuthUserAccount>()

        this.list = list.action

        this.loginId = loginId
        this.grantedRoles = grantedRoles.input
        this.resetTokenDestination = resetTokenDestination
        this.memo = memo
        this.validate = validate
        this.observe = observe
        this.convert = convert
        this.clear = clear

        this.clear()

        this.onSuccess((data) => {
            this.clear()
            list.handler.register(data)
        })
    }

    onSuccess(handler: (data: AuthUserAccount) => void): void {
        this.state.subscribe((state) => {
            switch (state.type) {
                case "success":
                    handler(state.entry)
                    break
            }
        })
    }

    async submit(): Promise<RegisterAuthUserAccountState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return registerUser(this.material, fields.value, this.post)
    }
}

type RegisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: RegisterAuthUserAccountError }>
    | Readonly<{ type: "success"; entry: AuthUserAccount }>
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

    post({ type: "success", entry: fields })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
