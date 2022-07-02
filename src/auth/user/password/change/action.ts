import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { initPasswordFieldAction } from "../input/action"
import { PasswordFieldAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardAction } from "../../../../z_vendor/getto-application/board/observe_board/action"
import { initRegisterField } from "../../../../z_lib/ui/register/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

import { ChangePasswordRemote, OverwritePasswordRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { ChangePasswordError, ChangePasswordFields, OverwritePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface ChangePasswordAction extends StatefulApplicationAction<ChangePasswordState> {
    readonly currentPassword: PasswordFieldAction
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): void
    submit(onSuccess: { (): void }): Promise<ChangePasswordState>
}

export type ChangePasswordState = ChangePasswordEvent

const initialState: ChangePasswordState = { type: "initial" }

export interface OverwritePasswordAction extends StatefulApplicationAction<OverwritePasswordState> {
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): void
    submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (): void },
    ): Promise<OverwritePasswordState>
}

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
    return new Action(material)
}

class Action implements ChangePasswordAction {
    readonly material: ChangePasswordMaterial
    readonly state: ApplicationStateAction<ChangePasswordState>
    readonly post: (state: ChangePasswordState) => ChangePasswordState

    readonly currentPassword: PasswordFieldAction
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: () => ConvertBoardResult<ChangePasswordFields>
    clear: () => void

    constructor(material: ChangePasswordMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post

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

        this.currentPassword = currentPassword
        this.newPassword = newPassword
        this.validate = validate
        this.observe = observe
        this.convert = convert
        this.clear = clear
    }

    async submit(onSuccess: { (): void }): Promise<ChangePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return changePassword(this.material, fields.value, onSuccess, this.post)
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
    onSuccess: { (): void },
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

    onSuccess()
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

export function initOverwritePasswordAction(
    material: OverwritePasswordMaterial,
): OverwritePasswordAction {
    return new OverwriteAction(material)
}

class OverwriteAction implements OverwritePasswordAction {
    readonly material: OverwritePasswordMaterial
    readonly state: ApplicationStateAction<OverwritePasswordState>
    readonly post: (state: OverwritePasswordState) => OverwritePasswordState

    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    convert: () => ConvertBoardResult<OverwritePasswordFields>
    clear: () => void

    constructor(material: OverwritePasswordMaterial) {
        const { state, post } = initApplicationStateAction({ initialState: initialOverwriteState })
        this.material = material
        this.state = state
        this.post = post

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

        const { validate, observe, clear } = initRegisterField(
            [["newPassword", newPassword]],
            convert,
        )

        this.newPassword = newPassword
        this.validate = validate
        this.observe = observe
        this.convert = convert
        this.clear = clear
    }

    async submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (): void },
    ): Promise<OverwritePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return overwritePassword(this.material, user, fields.value, onSuccess, this.post)
    }
}

type OverwritePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function overwritePassword<S>(
    { infra, config }: OverwritePasswordMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: OverwritePasswordFields,
    onSuccess: { (): void },
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

    onSuccess()
    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
