import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"

import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    AuthUserTextFieldAction,
    AuthPermissionGrantedFieldAction,
    initAuthUserTextFieldAction,
    initAuthPermissionGrantedFieldAction,
} from "../input/field/action"
import {
    initModifyField,
    modifyField,
    ModifyFieldHandler,
} from "../../../../common/util/modify/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"

import { ALL_AUTH_PERMISSIONS } from "../../../../x_content/permission"

import { ModifyAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { LoginId } from "../../login_id/kernel/data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

export interface ModifyAuthUserAccountAction {
    readonly state: ApplicationState<ModifyAuthUserAccountState>
    readonly memo: AuthUserTextFieldAction<"memo">
    readonly granted: AuthPermissionGrantedFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: ModifyAuthUserAccountEntry) => void): void

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
    const granted = initAuthPermissionGrantedFieldAction()

    const convert = (): ConvertBoardResult<ModifyAuthUserAccountFields> => {
        const result = {
            granted: granted.input.validate.check(),
            memo: memo.validate.check(),
        }
        if (!result.granted.valid || !result.memo.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                granted: result.granted.value,
                memo: result.memo.value,
            },
        }
    }

    const { validate, observe, editable, data, handler, reset } = initModifyField(
        [
            modifyField("memo", memo, (data: ModifyAuthUserAccountEntry) => data.memo),
            modifyField(
                "granted",
                granted.input,
                (data: ModifyAuthUserAccountEntry) => data.granted,
            ),
        ],
        convert,
    )

    granted.setOptions(ALL_AUTH_PERMISSIONS)

    onSuccess(() => {
        editable.close()
    })

    return {
        action: {
            state,

            memo,
            granted: granted.input,

            validate,
            observe,
            editable,

            onSuccess,
            reset,

            async submit(): Promise<ModifyAuthUserAccountState> {
                const element = data()
                if (!element.isLoad) {
                    return state.currentState()
                }

                const fields = convert()
                if (!fields.valid) {
                    return state.currentState()
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
