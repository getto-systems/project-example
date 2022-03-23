import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import {
    initInputGrantedRolesAction,
    initInputResetTokenDestinationAction,
    InputGrantedRolesAction,
    InputResetTokenDestinationAction,
} from "../input/action"
import {
    ValidateBoardAction,
    initValidateBoardAction,
} from "../../../../z_vendor/getto-application/board/validate_board/action"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

import { ModifyAuthUserAccountRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { AuthUserAccountBasket } from "../kernel/data"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"
import { BoardConverter } from "../../../../z_vendor/getto-application/board/kernel/infra"

export interface ModifyAuthUserAccountAction
    extends StatefulApplicationAction<ModifyAuthUserAccountState> {
    readonly grantedRoles: InputGrantedRolesAction
    readonly resetTokenDestination: InputResetTokenDestinationAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    reset(user: AuthUserAccountBasket): ModifyAuthUserAccountState
    submit(user: AuthUserAccountBasket): Promise<ModifyAuthUserAccountState>
}

const modifyAuthUserAccountFieldNames = ["resetTokenDestination"] as const
export type ModifyAuthUserAccountFieldName = typeof modifyAuthUserAccountFieldNames[number]

export type ModifyAuthUserAccountState = Readonly<{ type: "initial" }> | ModifyUserEvent

const initialState: ModifyAuthUserAccountState = { type: "initial" }

export type ModifyAuthUserAccountMaterial = Readonly<{
    infra: ModifyAuthUserAccountInfra
    config: ModifyAuthUserAccountConfig
}>

export type ModifyAuthUserAccountInfra = Readonly<{
    modifyUserRemote: ModifyAuthUserAccountRemote
}>

export type ModifyAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>

export function initModifyAuthUserAccountAction(
    material: ModifyAuthUserAccountMaterial,
): ModifyAuthUserAccountAction {
    return new Action(material)
}

class Action
    extends AbstractStatefulApplicationAction<ModifyAuthUserAccountState>
    implements ModifyAuthUserAccountAction
{
    readonly initialState = initialState

    readonly grantedRoles: InputGrantedRolesAction
    readonly resetTokenDestination: InputResetTokenDestinationAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: ModifyAuthUserAccountMaterial
    convert: BoardConverter<ModifyAuthUserAccountFields>

    constructor(material: ModifyAuthUserAccountMaterial) {
        super({
            terminate: () => {
                this.grantedRoles.terminate()
                this.resetTokenDestination.terminate()
                this.validate.terminate()
            },
        })
        this.material = material

        const fields = ["grantedRoles", "resetTokenDestination"] as const

        const grantedRoles = initInputGrantedRolesAction()
        const resetTokenDestination = initInputResetTokenDestinationAction()
        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                converter: (): ConvertBoardResult<ModifyAuthUserAccountFields> => {
                    const result = {
                        grantedRoles: grantedRoles.convert(),
                        resetTokenDestination: resetTokenDestination.checker.check(),
                    }
                    if (!result.resetTokenDestination.valid) {
                        return { valid: false }
                    }
                    return {
                        valid: true,
                        value: {
                            grantedRoles: result.grantedRoles,
                            resetTokenDestination: result.resetTokenDestination.value,
                        },
                    }
                },
            },
        )

        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.grantedRoles = grantedRoles.input
        this.resetTokenDestination = resetTokenDestination.input
        this.validate = validate
        this.observe = observe
        this.convert = () => validateChecker.get()

        this.resetTokenDestination.validate.subscriber.subscribe((result) =>
            validateChecker.update("resetTokenDestination", result.valid),
        )

        this.grantedRoles.observe.subscriber.subscribe((result) => {
            observeChecker.update("grantedRoles", result.hasChanged)
        })
        this.resetTokenDestination.observe.subscriber.subscribe((result) => {
            observeChecker.update("resetTokenDestination", result.hasChanged)
        })
    }

    reset(user: AuthUserAccountBasket): ModifyAuthUserAccountState {
        this.grantedRoles.reset(user)
        this.resetTokenDestination.reset(user)
        this.validate.clear()
        this.observe.clear()
        return this.post(this.initialState)
    }
    async submit(user: AuthUserAccountBasket): Promise<ModifyAuthUserAccountState> {
        return modifyUser(this.material, user, this.convert(), this.post)
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success"; data: AuthUserAccountBasket }>

async function modifyUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: AuthUserAccountBasket,
    fields: ConvertBoardResult<ModifyAuthUserAccountFields>,
    post: Post<ModifyUserEvent, S>,
): Promise<S> {
    if (!fields.valid) {
        return post({ type: "failed", err: { type: "validation-error" } })
    }

    post({ type: "try" })

    const { modifyUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        modifyUserRemote(user, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", data: response.value })
}

interface Post<E, S> {
    (event: E): S
}
