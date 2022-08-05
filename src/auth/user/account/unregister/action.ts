import {
    ApplicationState,
    initApplicationState,
    StatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"
import { initEditableDataHandler, ModifyFieldHandler } from "../../../../z_lib/ui/modify/action"

import { checkTakeLongtime } from "../../../../z_lib/ui/timer/helper"

import { UnregisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { UnregisterAuthUserAccountError } from "./data"
import { LoginId } from "../../login_id/kernel/data"
import { PrepareElementState } from "../../../../z_lib/ui/prepare/data"

export interface UnregisterAuthUserAccountAction
    extends StatefulApplicationAction<UnregisterAuthUserAccountState> {
    readonly editable: EditableBoardAction

    onSuccess(handler: (data: UnregisterAuthUserAccountEntry) => void): void

    data(): PrepareElementState<UnregisterAuthUserAccountEntry>

    submit(): Promise<UnregisterAuthUserAccountState>
}

export type UnregisterAuthUserAccountEntry = Readonly<{ loginId: LoginId }>

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
): Readonly<{
    action: UnregisterAuthUserAccountAction
    handler: ModifyFieldHandler<UnregisterAuthUserAccountEntry>
}> {
    const action = new Action(material)
    return { action, handler: action.handler }
}

class Action implements UnregisterAuthUserAccountAction {
    readonly material: UnregisterAuthUserAccountMaterial
    readonly state: ApplicationState<UnregisterAuthUserAccountState>
    readonly post: (state: UnregisterAuthUserAccountState) => UnregisterAuthUserAccountState

    readonly editable: EditableBoardAction

    readonly data: () => PrepareElementState<UnregisterAuthUserAccountEntry>
    readonly handler: ModifyFieldHandler<UnregisterAuthUserAccountEntry>

    constructor(material: UnregisterAuthUserAccountMaterial) {
        const { state, post } = initApplicationState({ initialState })
        this.material = material
        this.state = state
        this.post = post

        const { editable, data, handler } =
            initEditableDataHandler<UnregisterAuthUserAccountEntry>()

        this.editable = editable
        this.data = data
        this.handler = handler

        this.onSuccess(() => {
            this.editable.close()
        })
    }

    onSuccess(handler: (data: Readonly<{ loginId: LoginId }>) => void): void {
        this.state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.entry)
            }
        })
    }

    async submit(): Promise<UnregisterAuthUserAccountState> {
        const element = this.data()
        if (!element.isLoad) {
            return this.state.currentState()
        }

        return unregisterUser(this.material, element.data, this.post)
    }
}

type UnregisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: UnregisterAuthUserAccountError }>
    | Readonly<{ type: "success"; entry: UnregisterAuthUserAccountEntry }>

async function unregisterUser<S>(
    { infra, config }: UnregisterAuthUserAccountMaterial,
    user: Readonly<{ loginId: LoginId }>,
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

    return post({ type: "success", entry: user })
}

interface Post<E, S> {
    (event: E): S
}
