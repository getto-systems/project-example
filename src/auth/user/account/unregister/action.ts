import {
    ApplicationStateAction,
    initApplicationStateAction,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { checkTakeLongtime } from "../../../../z_lib/ui/timer/helper"

import { UnregisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { UnregisterAuthUserAccountError } from "./data"
import { LoginId } from "../../login_id/kernel/data"

export interface UnregisterAuthUserAccountAction
    extends StatefulApplicationAction<UnregisterAuthUserAccountState> {
    submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (): void },
    ): Promise<UnregisterAuthUserAccountState>
}

export type UnregisterAuthUserAccountState = Readonly<{ type: "initial" }> | UnregisterUserEvent

const initialState: UnregisterAuthUserAccountState = { type: "initial" }

export type UnregisterAuthUserAccountMaterial = Readonly<{
    infra: UnregisterAuthUserAccountInfra
    config: UnregisterAuthUserAccountConfig
}>

export type UnregisterAuthUserAccountInfra = Readonly<{
    unregisterUserRemote: UnregisterAuthUserAccountRemote
}>

export type UnregisterAuthUserAccountConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
}>

export function initUnregisterAuthUserAccountAction(
    material: UnregisterAuthUserAccountMaterial,
): UnregisterAuthUserAccountAction {
    return new Action(material)
}

class Action implements UnregisterAuthUserAccountAction {
    readonly material: UnregisterAuthUserAccountMaterial
    readonly state: ApplicationStateAction<UnregisterAuthUserAccountState>
    readonly post: (state: UnregisterAuthUserAccountState) => UnregisterAuthUserAccountState

    constructor(material: UnregisterAuthUserAccountMaterial) {
        const { state, post } = initApplicationStateAction({ initialState })
        this.material = material
        this.state = state
        this.post = post
    }

    async submit(
        user: Readonly<{ loginId: LoginId }>,
        onSuccess: { (): void },
    ): Promise<UnregisterAuthUserAccountState> {
        return unregisterUser(this.material, user, onSuccess, this.post)
    }
}

type UnregisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: UnregisterAuthUserAccountError }>
    | Readonly<{ type: "success" }>

async function unregisterUser<S>(
    { infra, config }: UnregisterAuthUserAccountMaterial,
    user: Readonly<{ loginId: LoginId }>,
    onSuccess: { (): void },
    post: Post<UnregisterUserEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { unregisterUserRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        unregisterUserRemote(user),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    onSuccess()
    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
