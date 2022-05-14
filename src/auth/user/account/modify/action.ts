import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime, ticker } from "../../../../z_lib/ui/timer/helper"

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
import { WaitTime } from "../../../../z_lib/ui/config/infra"

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
        onSuccess: { (data: ModifyAuthUserAccountFields): void },
    ): Promise<ModifyAuthUserAccountState>
}

export type ModifyAuthUserAccountState = ModifyUserEvent

const initialState: ModifyAuthUserAccountState = { type: "initial" }

export type ModifyAuthUserAccountMaterial = Readonly<{
    infra: ModifyAuthUserAccountInfra
    config: ModifyAuthUserAccountConfig
}>

export type ModifyAuthUserAccountInfra = Readonly<{
    modifyUserRemote: ModifyAuthUserAccountRemote
}>

export type ModifyAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
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

        const memo = initInputAuthUserMemoAction()
        const grantedRoles = initInputGrantedAuthRolesAction()

        const fields = ["memo", "grantedRoles"] as const
        const convert = (): ConvertBoardResult<ModifyAuthUserAccountFields> => {
            const result = {
                grantedRoles: grantedRoles.validate.check(),
                memo: memo.validate.check(),
            }
            if (!result.grantedRoles.valid || !result.memo.valid) {
                return { valid: false }
            }
            return {
                valid: true,
                value: {
                    grantedRoles: result.grantedRoles.value,
                    memo: result.memo.value,
                },
            }
        }

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.memo = memo
        this.grantedRoles = grantedRoles
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

    reset(user: ModifyAuthUserAccountFields): ModifyAuthUserAccountState {
        this.memo.reset(user.memo)
        this.grantedRoles.reset(user.grantedRoles)
        this.observe.clear()
        return this.currentState()
    }
    async submit(
        user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
        onSuccess: { (data: ModifyAuthUserAccountFields): void },
    ): Promise<ModifyAuthUserAccountState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
        }
        return modifyUser(this.material, user, fields.value, onSuccess, this.post)
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function modifyUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
    fields: ModifyAuthUserAccountFields,
    onSuccess: { (data: ModifyAuthUserAccountFields): void },
    post: Post<ModifyUserEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { modifyUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        modifyUserRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    onSuccess(fields)
    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
