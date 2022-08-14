import {
    ApplicationState,
    initApplicationState,
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
import {
    initModifyField,
    modifyField,
    ModifyFieldHandler,
} from "../../../../z_lib/ui/modify/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"

import { ALL_AUTH_ROLES } from "../../../../x_content/role"

import { ModifyAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { LoginId } from "../../login_id/kernel/data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { PrepareElementState } from "../../../../z_lib/ui/prepare/data"

export interface ModifyAuthUserAccountAction {
    readonly state: ApplicationState<ModifyAuthUserAccountState>
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly grantedRoles: AuthUserGrantedRolesFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: ModifyAuthUserAccountEntry) => void): void

    data(): PrepareElementState<ModifyAuthUserAccountEntry>
    reset(): void
    submit(): Promise<ModifyAuthUserAccountState>
}

export type ModifyAuthUserAccountEntry = Readonly<{ loginId: LoginId }> &
    ModifyAuthUserAccountFields

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

export function initModifyAuthUserAccountAction(material: ModifyAuthUserAccountMaterial): Readonly<{
    action: ModifyAuthUserAccountAction
    handler: ModifyFieldHandler<ModifyAuthUserAccountEntry>
}> {
    const { state, post } = initApplicationState({ initialState })

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

    const { validate, observe, editable, data, handler, reset } = initModifyField(
        [
            modifyField("memo", memo, (data: ModifyAuthUserAccountEntry) => data.memo),
            modifyField(
                "grantedRoles",
                grantedRoles.input,
                (data: ModifyAuthUserAccountEntry) => data.grantedRoles,
            ),
        ],
        convert,
    )

    grantedRoles.setOptions(ALL_AUTH_ROLES)

    onSuccess(() => {
        editable.close()
    })

    return {
        action: {
            state,

            memo,
            grantedRoles: grantedRoles.input,

            validate,
            observe,
            editable,

            onSuccess,
            data,
            reset,

            async submit(): Promise<ModifyAuthUserAccountState> {
                const element = data()
                if (!element.isLoad) {
                    return state.currentState()
                }

                const fields = convert()
                if (!fields.valid) {
                    return this.state.currentState()
                }

                return modifyUser(material, element.data, fields.value, post)
            },
        },
        handler,
    }

    function onSuccess(handler: (data: ModifyAuthUserAccountEntry) => void): void {
        state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.data)
            }
        })
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success"; data: ModifyAuthUserAccountEntry }>
    | Readonly<{ type: "initial" }>

async function modifyUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: ModifyAuthUserAccountEntry,
    fields: ModifyAuthUserAccountFields,
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

    post({ type: "success", data: { ...user, ...fields } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
