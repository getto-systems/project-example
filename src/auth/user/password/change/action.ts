import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"
import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"
import { initInputPasswordAction } from "../input/action"
import { initValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/validate_board/action"

import { InputPasswordAction } from "../input/action"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/validate_board/action"

import { ChangePasswordError, ChangePasswordFields } from "./data"
import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"

import { ChangePasswordRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { ValidateBoardChecker } from "../../../../../ui/vendor/getto-application/board/validate_board/infra"

export interface ChangePasswordAction extends ApplicationStateAction<ChangePasswordState> {
    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    open(): ChangePasswordState
    clear(): ChangePasswordState
    submit(): Promise<ChangePasswordState>
    close(): ChangePasswordState
}

export const changePasswordFieldNames = ["currentPassword", "newPassword"] as const
export type ChangePasswordFieldName = typeof changePasswordFieldNames[number]

export type ChangePasswordState =
    | Readonly<{ type: "initial-change-password" }>
    | Readonly<{ type: "input-password" }>
    | ChangePasswordEvent

export const initialChangePasswordState: ChangePasswordState = {
    type: "initial-change-password",
}

export type ChangePasswordConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export type ChangePasswordInfra = Readonly<{
    changePasswordRemote: ChangePasswordRemote
}>

export function initChangePasswordAction(
    config: ChangePasswordConfig,
    infra: ChangePasswordInfra,
): ChangePasswordAction {
    return new Action(config, infra)
}

class Action
    extends ApplicationAbstractStateAction<ChangePasswordState>
    implements ChangePasswordAction
{
    readonly initialState = initialChangePasswordState

    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    config: ChangePasswordConfig
    infra: ChangePasswordInfra
    checker: ValidateBoardChecker<ChangePasswordFieldName, ChangePasswordFields>

    constructor(config: ChangePasswordConfig, infra: ChangePasswordInfra) {
        super()
        this.config = config
        this.infra = infra

        const currentPassword = initInputPasswordAction()
        const newPassword = initInputPasswordAction()
        const { validate, checker } = initValidateBoardAction(
            {
                fields: changePasswordFieldNames,
            },
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
        this.checker = checker

        this.currentPassword.validate.subscriber.subscribe((result) =>
            checker.update("currentPassword", result.valid),
        )
        this.newPassword.validate.subscriber.subscribe((result) =>
            checker.update("newPassword", result.valid),
        )

        this.terminateHook(() => {
            this.currentPassword.terminate()
            this.newPassword.terminate()
            this.validate.terminate()
        })
    }

    open(): ChangePasswordState {
        this.clearInput()
        return this.post({ type: "input-password" })
    }
    clear(): ChangePasswordState {
        this.clearInput()
        return this.post({ type: "input-password" })
    }
    async submit(): Promise<ChangePasswordState> {
        return changePassword(this.config, this.infra, this.checker.get(), this.post)
    }
    close(): ChangePasswordState {
        this.clearInput()
        return this.post(this.initialState)
    }

    clearInput(): void {
        this.currentPassword.clear()
        this.newPassword.clear()
        this.validate.clear()
    }
}

type ChangePasswordEvent =
    | Readonly<{ type: "try-to-change-password" }>
    | Readonly<{ type: "take-longtime-to-change-password" }>
    | Readonly<{ type: "failed-to-change-password"; err: ChangePasswordError }>
    | Readonly<{ type: "succeed-to-change-password" }>

async function changePassword<S>(
    config: ChangePasswordConfig,
    infra: ChangePasswordInfra,
    fields: ConvertBoardResult<ChangePasswordFields>,
    post: Post<ChangePasswordEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed-to-change-password", err: { type: "validation-error" } })
    }

    post({ type: "try-to-change-password" })

    const { changePasswordRemote: changeRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        changeRemote(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-change-password" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-change-password", err: response.err })
    }

    return post({ type: "succeed-to-change-password" })
}

interface Post<E, S> {
    (event: E): S
}
