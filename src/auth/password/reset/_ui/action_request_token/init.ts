import { ApplicationAbstractStateAction } from "../../../../../../ui/vendor/getto-application/action/init"

import { requestResetToken } from "../request_token/method"

import { RequestResetTokenInfra } from "../request_token/infra"

import {
    RequestResetTokenMaterial,
    RequestResetTokenAction,
    RequestResetTokenState,
    initialRequestResetTokenState,
    ValidateRequestTokenAction,
    requestResetTokenFields,
} from "./action"

import { RequestResetTokenFields } from "../request_token/data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { initInputLoginIDAction } from "../../../../login_id/_ui/action_input/init"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/init"
import { initSignLink } from "../../../../_ui/common/nav/action_nav/init"

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

    readonly loginID = initInputLoginIDAction()
    readonly validate: ValidateRequestTokenAction

    material: RequestResetTokenMaterial

    constructor(material: RequestResetTokenMaterial) {
        super()
        this.material = material

        this.validate = initValidateBoardAction({
            fields: requestResetTokenFields,
            converter: (): ConvertBoardResult<RequestResetTokenFields> => {
                const loginIDResult = this.loginID.validate.get()
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

        this.loginID.validate.subscriber.subscribe(this.validate.updateValidateState("loginID"))

        this.terminateHook(() => {
            this.loginID.terminate()
            this.validate.terminate()
        })
    }

    clear(): void {
        this.loginID.clear()
        this.validate.clear()
    }
    submit(fields: ConvertBoardResult<RequestResetTokenFields>): Promise<RequestResetTokenState> {
        return this.material.requestToken(fields, this.post)
    }
}
