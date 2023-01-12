import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { initPasswordFieldAction } from "../input/action"
import { PasswordFieldAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { initRegisterField } from "../../../../common/util/register/action"
import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../../z_vendor/getto-application/board/editable/action"
import {
    initModifyField,
    modifyField,
    ModifyFieldHandler,
} from "../../../../common/util/modify/action"

import { checkTakeLongtime, ticker } from "../../../../common/util/timer/helper"

import { ChangePasswordRemote, OverwritePasswordRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { ChangePasswordError, ChangePasswordFields, OverwritePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface ChangePasswordAction {
    readonly state: ApplicationState<ChangePasswordState>
    readonly currentPassword: PasswordFieldAction
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    edit(): void
    clear(): void
    submit(): Promise<ChangePasswordState>
}

export type ChangePasswordState = ChangePasswordEvent

const initialState: ChangePasswordState = { type: "initial" }

export interface OverwritePasswordAction {
    readonly state: ApplicationState<OverwritePasswordState>
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: OverwritePasswordEntry) => void): void

    reset(): void
    submit(): Promise<OverwritePasswordState>
}

export type OverwritePasswordEntry = Readonly<{ loginId: LoginId }>

export type OverwritePasswordState = OverwritePasswordEvent

const initialOverwriteState: OverwritePasswordState = { type: "initial" }

export type ChangePasswordMaterial = Readonly<{
    infra: ChangePasswordInfra
    config: ChangePasswordConfig
}>

export type ChangePasswordInfra = Readonly<{
    changePasswordRemote: ChangePasswordRemote
}>

export type ChangePasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initChangePasswordAction(material: ChangePasswordMaterial): ChangePasswordAction {
    const { state, post } = initApplicationState({ initialState })
    const editable = initEditableBoardAction()

    const currentPassword = initPasswordFieldAction()
    const newPassword = initPasswordFieldAction()

    const convert = (): ConvertBoardResult<ChangePasswordFields> => {
        const result = {
            currentPassword: currentPassword.validate.check(),
            newPassword: newPassword.validate.check(),
        }
        if (!result.currentPassword.valid || !result.newPassword.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                currentPassword: result.currentPassword.value,
                newPassword: result.newPassword.value,
            },
        }
    }

    const { validate, observe, clear } = initRegisterField(
        [
            ["newPassword", newPassword],
            ["currentPassword", currentPassword],
        ],
        convert,
    )

    onSuccess(() => {
        editable.close()
    })

    return {
        state,

        currentPassword,
        newPassword,

        validate,
        observe,
        editable,

        clear,

        edit(): void {
            editable.open()
            clear()
        },
        async submit(): Promise<ChangePasswordState> {
            const fields = convert()
            if (!fields.valid) {
                return state.currentState()
            }
            return changePassword(material, fields.value, post)
        },
    }

    function onSuccess(handler: () => void): void {
        state.subscribe((state) => {
            switch (state.type) {
                case "success":
                    handler()
                    break
            }
        })
    }
}

type ChangePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function changePassword<S>(
    { infra, config }: ChangePasswordMaterial,
    fields: ChangePasswordFields,
    post: Post<ChangePasswordEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { changePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        changePasswordRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

export type OverwritePasswordMaterial = Readonly<{
    infra: OverwritePasswordInfra
    config: OverwritePasswordConfig
}>

export type OverwritePasswordInfra = Readonly<{
    overwritePasswordRemote: OverwritePasswordRemote
}>

export type OverwritePasswordConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initOverwritePasswordAction(material: OverwritePasswordMaterial): Readonly<{
    action: OverwritePasswordAction
    handler: ModifyFieldHandler<OverwritePasswordEntry>
}> {
    const { state, post } = initApplicationState({ initialState: initialOverwriteState })

    const newPassword = initPasswordFieldAction()

    const convert = (): ConvertBoardResult<OverwritePasswordFields> => {
        const result = {
            newPassword: newPassword.validate.check(),
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

    const { validate, observe, editable, data, handler, reset } = initModifyField(
        [modifyField("newPassword", newPassword, (_data: OverwritePasswordEntry) => "")],
        convert,
    )

    onSuccess(() => {
        editable.close()
    })

    return {
        action: {
            state,

            newPassword,

            validate,
            observe,
            editable,

            reset,

            onSuccess,

            async submit(): Promise<OverwritePasswordState> {
                const element = data()
                if (!element.isLoad) {
                    return state.currentState()
                }

                const fields = convert()
                if (!fields.valid) {
                    return state.currentState()
                }

                return overwritePassword(material, element.data, fields.value, post)
            },
        },
        handler,
    }

    function onSuccess(handler: (data: Readonly<{ loginId: LoginId }>) => void): void {
        state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.data)
            }
        })
    }
}

type OverwritePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success"; data: OverwritePasswordEntry }>
    | Readonly<{ type: "initial" }>

async function overwritePassword<S>(
    { infra, config }: OverwritePasswordMaterial,
    user: OverwritePasswordEntry,
    fields: OverwritePasswordFields,
    post: Post<OverwritePasswordEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { overwritePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        overwritePasswordRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: user })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
