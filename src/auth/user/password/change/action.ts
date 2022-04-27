import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { initInputPasswordAction } from "../input/action"
import { initValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import { InputPasswordAction } from "../input/action"
import { ValidateBoardAction } from "../../../../z_vendor/getto-application/board/validate_board/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { ChangePasswordRemote, OverridePasswordRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { ChangePasswordError, ChangePasswordFields, OverridePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface ChangePasswordAction extends StatefulApplicationAction<ChangePasswordState> {
    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): ChangePasswordState
    submit(): Promise<ChangePasswordState>
}

export type ChangePasswordState = Readonly<{ type: "initial" }> | ChangePasswordEvent

const initialState: ChangePasswordState = { type: "initial" }

export interface OverridePasswordAction extends StatefulApplicationAction<OverridePasswordState> {
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    clear(): OverridePasswordState
    submit(user: Readonly<{ loginId: LoginId }>): Promise<OverridePasswordState>
}

export type OverridePasswordState = Readonly<{ type: "initial" }> | OverridePasswordEvent

const initialOverrideState: OverridePasswordState = { type: "initial" }

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
    readonly observe: ObserveBoardAction

    material: ChangePasswordMaterial
    convert: { (): ConvertBoardResult<ChangePasswordFields> }

    constructor(material: ChangePasswordMaterial) {
        super({
            terminate: () => {
                this.currentPassword.terminate()
                this.newPassword.terminate()
                this.validate.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const currentPassword = initInputPasswordAction()
        const newPassword = initInputPasswordAction()

        const fields = ["currentPassword", "newPassword"] as const
        const convert = (): ConvertBoardResult<ChangePasswordFields> => {
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
        }

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.currentPassword = currentPassword.input
        this.newPassword = newPassword.input
        this.validate = validate
        this.observe = observe
        this.convert = convert

        this.currentPassword.validate.subscriber.subscribe((result) =>
            validateChecker.update("currentPassword", result.valid),
        )
        this.currentPassword.observe.subscriber.subscribe((result) =>
            observeChecker.update("currentPassword", result.hasChanged),
        )
        this.newPassword.validate.subscriber.subscribe((result) =>
            validateChecker.update("newPassword", result.valid),
        )
        this.newPassword.observe.subscriber.subscribe((result) =>
            observeChecker.update("newPassword", result.hasChanged),
        )
    }

    clear(): ChangePasswordState {
        this.currentPassword.clear()
        this.newPassword.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(): Promise<ChangePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return changePassword(this.material, fields.value, this.post)
    }
}

type ChangePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success" }>

async function changePassword<S>(
    { infra, config }: ChangePasswordMaterial,
    fields: ChangePasswordFields,
    post: Post<ChangePasswordEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { changePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        changePasswordRemote(fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success" })
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
    readonly observe: ObserveBoardAction

    material: OverridePasswordMaterial
    convert: { (): ConvertBoardResult<OverridePasswordFields> }

    constructor(material: OverridePasswordMaterial) {
        super({
            terminate: () => {
                this.newPassword.terminate()
                this.validate.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const fields = ["newPassword"] as const

        const newPassword = initInputPasswordAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                convert: (): ConvertBoardResult<OverridePasswordFields> => {
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
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.newPassword = newPassword.input
        this.validate = validate
        this.observe = observe
        this.convert = () => validateChecker.get()

        this.newPassword.validate.subscriber.subscribe((result) =>
            validateChecker.update("newPassword", result.valid),
        )
        this.newPassword.observe.subscriber.subscribe((result) =>
            observeChecker.update("newPassword", result.hasChanged),
        )
    }

    clear(): OverridePasswordState {
        this.newPassword.clear()
        this.validate.clear()
        return this.post(this.initialState)
    }
    async submit(user: Readonly<{ loginId: LoginId }>): Promise<OverridePasswordState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return overridePassword(this.material, user, fields.value, this.post)
    }
}

type OverridePasswordEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangePasswordError }>
    | Readonly<{ type: "success" }>

async function overridePassword<S>(
    { infra, config }: OverridePasswordMaterial,
    user: Readonly<{ loginId: LoginId }>,
    fields: OverridePasswordFields,
    post: Post<OverridePasswordEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { overridePasswordRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        overridePasswordRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
