import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { initPasswordFieldAction } from "../input/action"
import { initValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { PasswordFieldAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"

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

    clear(): ChangePasswordState
    submit(onSuccess: { (): void }): Promise<ChangePasswordState>
}

export type ChangePasswordState = ChangePasswordEvent

const initialState: ChangePasswordState = { type: "initial" }

export interface OverwritePasswordAction extends StatefulApplicationAction<OverwritePasswordState> {
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): OverwritePasswordState
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

class Action
    extends AbstractStatefulApplicationAction<ChangePasswordState>
    implements ChangePasswordAction
{
    readonly initialState = initialState

    readonly currentPassword: PasswordFieldAction
    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: ChangePasswordMaterial
    convert: { (): ConvertBoardResult<ChangePasswordFields> }

    constructor(material: ChangePasswordMaterial) {
        super()
        this.material = material

        const currentPassword = initPasswordFieldAction()
        const newPassword = initPasswordFieldAction()

        const fields = ["currentPassword", "newPassword"] as const
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

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.currentPassword = currentPassword
        this.newPassword = newPassword
        this.validate = validate
        this.observe = observe
        this.convert = convert

        fields.forEach((field) => {
            this[field].validate.subscriber.subscribe((state) => {
                validateChecker.update(field, state)
            })
            this[field].observe.subscriber.subscribe((result) => {
                observeChecker.update(field, result.hasChanged)
            })
        })
    }

    clear(): ChangePasswordState {
        this.currentPassword.clear()
        this.newPassword.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(onSuccess: { (): void }): Promise<ChangePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
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

class OverwriteAction
    extends AbstractStatefulApplicationAction<OverwritePasswordState>
    implements OverwritePasswordAction
{
    readonly initialState = initialOverwriteState

    readonly newPassword: PasswordFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: OverwritePasswordMaterial
    convert: { (): ConvertBoardResult<OverwritePasswordFields> }

    constructor(material: OverwritePasswordMaterial) {
        super()
        this.material = material

        const fields = ["newPassword"] as const

        const newPassword = initPasswordFieldAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                convert: (): ConvertBoardResult<OverwritePasswordFields> => {
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
                },
            },
        )
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.newPassword = newPassword
        this.validate = validate
        this.observe = observe
        this.convert = () => validateChecker.get()

        fields.forEach((field) => {
            this[field].validate.subscriber.subscribe((state) => {
                validateChecker.update(field, state)
            })
            this[field].observe.subscriber.subscribe((result) => {
                observeChecker.update(field, result.hasChanged)
            })
        })
    }

    clear(): OverwritePasswordState {
        this.newPassword.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (): void },
    ): Promise<OverwritePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
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
