import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import {
    initInputGrantedAuthRolesAction,
    InputGrantedAuthRolesAction,
} from "../input/granted_roles/action"
import { initInputAuthUserMemoAction, InputAuthUserMemoAction } from "../input/memo/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"
import {
    initValidateBoardAction,
    ValidateBoardAction,
} from "../../../../z_vendor/getto-application/board/validate_board/action"

import { ModifyAuthUserAccountRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { AuthRole } from "../../kernel/data"
import { LoginId } from "../../login_id/kernel/data"
import { ConvertBoardResult } from "../../../../z_vendor/getto-application/board/kernel/data"

export interface ModifyAuthUserAccountAction
    extends StatefulApplicationAction<ModifyAuthUserAccountState> {
    readonly memo: InputAuthUserMemoAction
    readonly grantedRoles: InputGrantedAuthRolesAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    reset(user: Readonly<{ grantedRoles: readonly AuthRole[] }>): ModifyAuthUserAccountState
    submit(
        user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
    ): Promise<ModifyAuthUserAccountState>
}

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

    readonly memo: InputAuthUserMemoAction
    readonly grantedRoles: InputGrantedAuthRolesAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: ModifyAuthUserAccountMaterial

    readonly convert: {
        (): ConvertBoardResult<ModifyAuthUserAccountFields>
    }

    constructor(material: ModifyAuthUserAccountMaterial) {
        super({
            terminate: () => {
                this.grantedRoles.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const fields = ["memo", "grantedRoles"] as const

        const memo = initInputAuthUserMemoAction()
        const grantedRoles = initInputGrantedAuthRolesAction()

        const convert = (): ConvertBoardResult<ModifyAuthUserAccountFields> => {
            const result = {
                grantedRoles: grantedRoles.convert(),
                memo: memo.convert(),
            }
            if (!result.memo.valid) {
                return { valid: false }
            }
            return {
                valid: true,
                value: {
                    grantedRoles: result.grantedRoles,
                    memo: result.memo.value,
                },
            }
        }

        const { validate, validateChecker } = initValidateBoardAction(
            { fields },
            {
                // TODO converter => convert がいいかな
                converter: convert,
            },
        )
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.memo = memo.input
        this.grantedRoles = grantedRoles.input
        this.validate = validate
        this.observe = observe
        this.convert = convert

        this.memo.validate.subscriber.subscribe((result) => {
            validateChecker.update("memo", result.valid)
        })
        this.memo.observe.subscriber.subscribe((result) => {
            observeChecker.update("memo", result.hasChanged)
        })
        this.grantedRoles.observe.subscriber.subscribe((result) => {
            observeChecker.update("grantedRoles", result.hasChanged)
        })
    }

    reset(user: ModifyAuthUserAccountFields): ModifyAuthUserAccountState {
        this.memo.reset(user.memo)
        this.grantedRoles.reset(user.grantedRoles)
        this.observe.clear()
        return this.currentState()
    }
    async submit(
        user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
    ): Promise<ModifyAuthUserAccountState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return modifyUser(this.material, user, fields.value, this.post)
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success"; data: ModifyAuthUserAccountFields }>

async function modifyUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
    fields: ModifyAuthUserAccountFields,
    post: Post<ModifyUserEvent, S>,
): Promise<S> {
    post({ type: "try" })

    const { modifyUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        modifyUserRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    return post({ type: "success", data: fields })
}

interface Post<E, S> {
    (event: E): S
}
