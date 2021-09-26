import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"

import { initInputPasswordAction } from "../action_input/init"
import { initValidateBoardAction } from "../../../../ui/vendor/getto-application/board/action_validate_board/init"

import { changePassword } from "../change/method"

import { ChangePasswordInfra } from "../change/infra"

import {
    ChangePasswordMaterial,
    ChangePasswordAction,
    ChangePasswordState,
    initialChangePasswordState,
    changePasswordFieldNames,
    ChangePasswordFieldName,
} from "./action"
import { ValidateBoardAction } from "../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { ChangePasswordFields } from "../change/data"
import { ConvertBoardResult } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { InputPasswordAction } from "../action_input/action"
import { ValidateBoardChecker } from "../../../../ui/vendor/getto-application/board/validate_board/infra"

export type ChangePasswordActionInfra = Readonly<{
    change: ChangePasswordInfra
}>

export function initChangePasswordMaterial(
    infra: ChangePasswordActionInfra,
): ChangePasswordMaterial {
    return {
        change: changePassword(infra.change),
    }
}

export function initChangePasswordAction(material: ChangePasswordMaterial): ChangePasswordAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<ChangePasswordState>
    implements ChangePasswordAction
{
    readonly initialState = initialChangePasswordState

    readonly currentPassword: InputPasswordAction
    readonly newPassword: InputPasswordAction
    readonly validate: ValidateBoardAction

    material: ChangePasswordMaterial
    checker: ValidateBoardChecker<ChangePasswordFieldName, ChangePasswordFields>

    constructor(material: ChangePasswordMaterial) {
        super()
        this.material = material

        const currentPassword = initInputPasswordAction()
        const newPassword = initInputPasswordAction()
        const { validate, checker } = initValidateBoardAction({
            fields: changePasswordFieldNames,
            converter: (): ConvertBoardResult<ChangePasswordFields> => {
                const result = {
                    currentPassword: currentPassword.checker.get(),
                    newPassword: newPassword.checker.get(),
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
        })

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
        return this.material.change(this.checker.get(), this.post)
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
