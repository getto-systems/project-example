import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/init"

import { requestResetToken } from "../request_token/method"

import { RequestResetTokenInfra } from "../request_token/infra"

import {
    RequestResetTokenProfileMaterial,
    RequestResetTokenProfileAction,
    RequestResetTokenProfileState,
    initialRequestResetTokenProfileState,
    requestResetTokenProfileFieldNames,
    RequestResetTokenProfileFieldName,
} from "./action"

import { RequestResetTokenFields } from "../request_token/data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { initInputLoginIDAction } from "../../../login_id/input/action_input/init"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/init"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { InputLoginIDAction } from "../../../login_id/input/action_input/action"
import { ValidateBoardChecker } from "../../../../../../ui/vendor/getto-application/board/validate_board/infra"

export function initRequestResetTokenProfileMaterial(
    infra: RequestResetTokenInfra,
): RequestResetTokenProfileMaterial {
    return {
        requestToken: requestResetToken(infra),
    }
}

export function initRequestResetTokenProfileAction(
    material: RequestResetTokenProfileMaterial,
): RequestResetTokenProfileAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<RequestResetTokenProfileState>
    implements RequestResetTokenProfileAction
{
    readonly initialState = initialRequestResetTokenProfileState

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    material: RequestResetTokenProfileMaterial
    checker: ValidateBoardChecker<RequestResetTokenProfileFieldName, RequestResetTokenFields>

    constructor(material: RequestResetTokenProfileMaterial) {
        super()
        this.material = material

        const loginID = initInputLoginIDAction()

        const { validate, checker } = initValidateBoardAction({
            fields: requestResetTokenProfileFieldNames,
            converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                const loginIDResult = loginID.checker.get()
                if (!loginIDResult.valid) {
                    return { valid: false }
                }
                return {
                    valid: true,
                    value: {
                        loginID: loginIDResult.value,
                    },
                }
            },
        })

        this.loginID = loginID.input
        this.validate = validate
        this.checker = checker

        this.loginID.validate.subscriber.subscribe((result) =>
            checker.update("loginID", result.valid),
        )

        this.terminateHook(() => {
            this.loginID.terminate()
            this.validate.terminate()
        })
    }

    open(): RequestResetTokenProfileState {
        this.clearInput()
        return this.post({ type: "input-login-id" })
    }
    clear(): RequestResetTokenProfileState {
        this.clearInput()
        return this.post({ type: "input-login-id" })
    }
    submit(): Promise<RequestResetTokenProfileState> {
        return this.material.requestToken(this.checker.get(), this.post)
    }
    close(): RequestResetTokenProfileState {
        this.clearInput()
        return this.post(this.initialState)
    }

    clearInput(): void {
        this.loginID.clear()
        this.validate.clear()
    }
}
