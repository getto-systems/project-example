import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { requestResetToken } from "../request_token/method"

import { RequestResetTokenInfra } from "../request_token/infra"

import {
    RequestResetTokenMaterial,
    RequestResetTokenAction,
    RequestResetTokenState,
    initialRequestResetTokenState,
    requestResetTokenFieldNames,
    RequestResetTokenFieldName,
} from "./action"

import { RequestResetTokenFields } from "../request_token/data"
import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { initInputLoginIDAction } from "../../../user/login_id/input/action_input/init"
import { initValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/init"
import { initSignLink } from "../../../_ui/common/nav/action_nav/init"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { InputLoginIDAction } from "../../../user/login_id/input/action_input/action"
import { ValidateBoardChecker } from "../../../../../ui/vendor/getto-application/board/validate_board/infra"

export function initRequestResetTokenMaterial(
    infra: RequestResetTokenInfra,
): RequestResetTokenMaterial {
    return {
        requestToken: requestResetToken(infra),
    }
}

export function initRequestResetTokenAction(
    material: RequestResetTokenMaterial,
): RequestResetTokenAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<RequestResetTokenState>
    implements RequestResetTokenAction
{
    readonly initialState = initialRequestResetTokenState

    readonly link = initSignLink()

    readonly loginID: InputLoginIDAction
    readonly validate: ValidateBoardAction

    material: RequestResetTokenMaterial
    checker: ValidateBoardChecker<RequestResetTokenFieldName, RequestResetTokenFields>

    constructor(material: RequestResetTokenMaterial) {
        super()
        this.material = material

        const loginID = initInputLoginIDAction()

        const { validate, checker } = initValidateBoardAction({
            fields: requestResetTokenFieldNames,
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

    clear(): void {
        this.loginID.clear()
        this.validate.clear()
    }
    submit(): Promise<RequestResetTokenState> {
        return this.material.requestToken(this.checker.get(), this.post)
    }
}
