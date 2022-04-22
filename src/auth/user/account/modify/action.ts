import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { initInputGrantedRolesAction, InputGrantedRolesAction } from "../input/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../../z_vendor/getto-application/board/observe_board/action"

import { ModifyAuthUserAccountRemote } from "./infra"
import { DelayTime } from "../../../../z_lib/ui/config/infra"

import { ModifyAuthUserAccountError, ModifyAuthUserAccountFields } from "./data"
import { AuthRole } from "../../kernel/data"
import { LoginId } from "../../login_id/kernel/data"

export interface ModifyAuthUserAccountAction
    extends StatefulApplicationAction<ModifyAuthUserAccountState> {
    readonly grantedRoles: InputGrantedRolesAction
    readonly observe: ObserveBoardAction

    reset(user: Readonly<{ grantedRoles: readonly AuthRole[] }>): ModifyAuthUserAccountState
    submit(
        user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>,
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

    readonly grantedRoles: InputGrantedRolesAction
    readonly observe: ObserveBoardAction

    material: ModifyAuthUserAccountMaterial
    convert: { (): ModifyAuthUserAccountFields }

    constructor(material: ModifyAuthUserAccountMaterial) {
        super({
            terminate: () => {
                this.grantedRoles.terminate()
                this.observe.terminate()
            },
        })
        this.material = material

        const fields = ["grantedRoles"] as const

        const grantedRoles = initInputGrantedRolesAction()

        const { observe, observeChecker } = initObserveBoardAction({ fields })

        this.grantedRoles = grantedRoles.input
        this.observe = observe
        this.convert = (): ModifyAuthUserAccountFields => ({
            grantedRoles: grantedRoles.convert(),
        })

        this.grantedRoles.observe.subscriber.subscribe((result) => {
            observeChecker.update("grantedRoles", result.hasChanged)
        })
    }

    reset(user: Readonly<{ grantedRoles: readonly AuthRole[] }>): ModifyAuthUserAccountState {
        this.grantedRoles.reset(user.grantedRoles)
        this.observe.clear()
        return this.currentState()
    }
    async submit(
        user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>,
    ): Promise<ModifyAuthUserAccountState> {
        return modifyUser(this.material, user, this.convert(), this.post)
    }
}

type ModifyUserEvent =
    | Readonly<{ type: "try" }>
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: ModifyAuthUserAccountError }>
    | Readonly<{ type: "success"; data: ModifyAuthUserAccountFields }>

async function modifyUser<S>(
    { infra, config }: ModifyAuthUserAccountMaterial,
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>,
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
