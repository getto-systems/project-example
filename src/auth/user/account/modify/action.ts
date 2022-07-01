import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    AuthUserTextFieldAction,
    AuthUserGrantedRolesFieldAction,
    initAuthUserTextFieldAction,
    initAuthUserGrantedRolesFieldAction,
} from "../input/field/action"
import { initModifyField, modifyField } from "../../../../z_lib/ui/modify/action"

import { ALL_AUTH_ROLES } from "../../../../x_content/role"

import { ModifyAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { AuthRole } from "../../kernel/data"
import { LoginId } from "../../login_id/kernel/data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

export interface ModifyAuthUserAccountAction
    extends StatefulApplicationAction<ModifyAuthUserAccountState> {
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    reset(user: Readonly<{ grantedRoles: readonly AuthRole[] }>): void
    submit(
        user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
        onSuccess: { (data: ModifyAuthUserAccountFields): void },
    ): Promise<ModifyAuthUserAccountState>
}

export type ModifyAuthUserAccountState = ModifyUserEvent

const initialState: ModifyAuthUserAccountState = { type: "initial" }

export type ModifyAuthUserAccountMaterial = Readonly<{
    infra: ModifyAuthUserAccountInfra
    config: ModifyAuthUserAccountConfig
}>

export type ModifyAuthUserAccountInfra = Readonly<{
    modifyUserRemote: ModifyAuthUserAccountRemote
}>

export type ModifyAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initModifyAuthUserAccountAction(
    material: ModifyAuthUserAccountMaterial,
): ModifyAuthUserAccountAction {
    return new Action(material)
}

class Action implements ModifyAuthUserAccountAction {
    readonly material: ModifyAuthUserAccountMaterial
    readonly state: ApplicationStateAction<ModifyAuthUserAccountState>
    readonly post: (state: ModifyAuthUserAccountState) => ModifyAuthUserAccountState

    readonly memo: AuthUserTextFieldAction<"memo">
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: () => ConvertBoardResult<ModifyAuthUserAccountFields>
    reset: (user: ModifyAuthUserAccountFields) => void

    constructor(material: ModifyAuthUserAccountMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

        const memo = initAuthUserTextFieldAction("memo")
        const grantedRoles = initAuthUserGrantedRolesFieldAction()

        const convert = (): ConvertBoardResult<ModifyAuthUserAccountFields> => {
            const result = {
                grantedRoles: grantedRoles.input.validate.check(),
                memo: memo.validate.check(),
            }
            if (!result.grantedRoles.valid || !result.memo.valid) {
                return { valid: false }
            }
            return {
                valid: true,
                value: {
                    grantedRoles: result.grantedRoles.value,
                    memo: result.memo.value,
                },
            }
        }

        const { validate, observe, reset } = initModifyField(
            [
                modifyField("memo", memo, (data: ModifyAuthUserAccountFields) => data.memo),
                modifyField(
                    "grantedRoles",
                    grantedRoles.input,
                    (data: ModifyAuthUserAccountFields) => data.grantedRoles,
                ),
            ],
            convert,
        )

        grantedRoles.setOptions(ALL_AUTH_ROLES)

        this.memo = memo
        this.grantedRoles = grantedRoles.input
        this.validate = validate
        this.observe = observe
        this.convert = convert
        this.reset = reset
    }

    async submit(
        user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
        onSuccess: { (data: ModifyAuthUserAccountFields): void },
    ): Promise<ModifyAuthUserAccountState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return modifyUser(this.material, user, fields.value, onSuccess, this.post)
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function modifyUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
    fields: ModifyAuthUserAccountFields,
    onSuccess: { (data: ModifyAuthUserAccountFields): void },
    post: Post<ModifyUserEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { modifyUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        modifyUserRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    onSuccess(fields)
    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
