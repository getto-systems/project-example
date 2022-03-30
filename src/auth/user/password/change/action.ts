import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { initInputPasswordAction } from "../input/action"
import { initValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { InputPasswordAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { ChangePasswordRemote, OverridePasswordRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { BoardConverter } from "../../../../z_vendor/getto-application/board/kernel/infra"

import { ChangePasswordError, ChangePasswordFields, OverridePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginId } from "../../login_id/input/data"

export interface ChangePasswordAction extends StatefulApplicationAction<ChangePasswordState> {
    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): ChangePasswordState
    submit(): Promise<ChangePasswordState>
}

export type ChangePasswordState =
    | Readonly<{ type: "initial-change-password" }>
    | ChangePasswordEvent

const initialState: ChangePasswordState = { type: "initial-change-password" }

export interface OverridePasswordAction extends StatefulApplicationAction<OverridePasswordState> {
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): OverridePasswordState
    submit(user: Readonly<{ loginId: LoginId }>): Promise<OverridePasswordState>
}

export type OverridePasswordState =
    | Readonly<{ type: "initial-override-password" }>
    | OverridePasswordEvent

const initialOverrideState: OverridePasswordState = { type: "initial-override-password" }

export type ChangePasswordMaterial = Readonly<{
    infra: ChangePasswordInfra
    config: ChangePasswordConfig
}>

export type ChangePasswordInfra = Readonly<{
    changePasswordRemote: ChangePasswordRemote
}>

export type ChangePasswordConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initChangePasswordAction(material: ChangePasswordMaterial): ChangePasswordAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<ChangePasswordState>
    implements ChangePasswordAction
{
    readonly initialState = initialState

    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    material: ChangePasswordMaterial
    convert: BoardConverter<ChangePasswordFields>

    constructor(material: ChangePasswordMaterial) {
        super({
            terminate: () => {
                this.currentPassword.terminate()
                this.newPassword.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const fields = ["currentPassword", "newPassword"] as const

        const currentPassword = initInputPasswordAction()
        const newPassword = initInputPasswordAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<ChangePasswordFields> => {
                    const result = {
                        currentPassword: currentPassword.checker.check(),
                        newPassword: newPassword.checker.check(),
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
                },
            },
        )

        this.currentPassword = currentPassword.input
        this.newPassword = newPassword.input
        this.validate = validate
        this.convert = () => validateChecker.get()

        this.currentPassword.validate.subscriber.subscribe((result) =>
            validateChecker.update("currentPassword", result.valid),
        )
        this.newPassword.validate.subscriber.subscribe((result) =>
            validateChecker.update("newPassword", result.valid),
        )
    }

    clear(): ChangePasswordState {
        this.currentPassword.clear()
        this.newPassword.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(): Promise<ChangePasswordState> {
        return changePassword(this.material, this.convert(), this.post)
    }
}

type ChangePasswordEvent =
    | Readonly<{ type: "try-to-change-password" }>
    | Readonly<{ type: "take-longtime-to-change-password" }>
    | Readonly<{ type: "failed-to-change-password"; err: ChangePasswordError }>
    | Readonly<{ type: "succeed-to-change-password" }>

async function changePassword<S>(
    { infra, config }: ChangePasswordMaterial,
    fields: ConvertBoardResult<ChangePasswordFields>,
    post: Post<ChangePasswordEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed-to-change-password", err: { type: "validation-error" } })
    }

    post({ type: "try-to-change-password" })

    const { changePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        changePasswordRemote(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-change-password" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-change-password", err: response.err })
    }

    return post({ type: "succeed-to-change-password" })
}

export type OverridePasswordMaterial = Readonly<{
    infra: OverridePasswordInfra
    config: OverridePasswordConfig
}>

export type OverridePasswordInfra = Readonly<{
    overridePasswordRemote: OverridePasswordRemote
}>

export type OverridePasswordConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initOverridePasswordAction(
    material: OverridePasswordMaterial,
): OverridePasswordAction {
    return new OverrideAction(material)
}

class OverrideAction
    extends AbstractStatefulApplicationAction<OverridePasswordState>
    implements OverridePasswordAction
{
    readonly initialState = initialOverrideState

    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    material: OverridePasswordMaterial
    convert: BoardConverter<OverridePasswordFields>

    constructor(material: OverridePasswordMaterial) {
        super({
            terminate: () => {
                this.newPassword.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const fields = ["newPassword"] as const

        const newPassword = initInputPasswordAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<OverridePasswordFields> => {
                    const result = {
                        newPassword: newPassword.checker.check(),
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

        this.newPassword = newPassword.input
        this.validate = validate
        this.convert = () => validateChecker.get()

        this.newPassword.validate.subscriber.subscribe((result) =>
            validateChecker.update("newPassword", result.valid),
        )
    }

    clear(): OverridePasswordState {
        this.newPassword.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(user: Readonly<{ loginId: LoginId }>): Promise<OverridePasswordState> {
        return overridePassword(this.material, user, this.convert(), this.post)
    }
}

type OverridePasswordEvent =
    | Readonly<{ type: "try-to-override-password" }>
    | Readonly<{ type: "take-longtime-to-override-password" }>
    | Readonly<{ type: "failed-to-override-password"; err: ChangePasswordError }>
    | Readonly<{ type: "succeed-to-override-password" }>

async function overridePassword<S>(
    { infra, config }: OverridePasswordMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: ConvertBoardResult<OverridePasswordFields>,
    post: Post<OverridePasswordEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed-to-override-password", err: { type: "validation-error" } })
    }

    post({ type: "try-to-override-password" })

    const { overridePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        overridePasswordRemote(user, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-override-password" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-override-password", err: response.err })
    }

    return post({ type: "succeed-to-override-password" })
}

interface Post<E, S> {
    (event: E): S
}
