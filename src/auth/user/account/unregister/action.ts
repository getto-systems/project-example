import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"
import { initEditableDataHandler, ModifyFieldHandler } from "../../../../z_lib/ui/modify/action"

import { checkTakeLongtime } from "../../../../z_lib/ui/timer/helper"

import { UnregisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../z_lib/ui/config/infra"

import { UnregisterAuthUserAccountError } from "./data"
import { LoginId } from "../../login_id/kernel/data"
import { PrepareElementState } from "../../../../z_lib/ui/prepare/data"

export interface UnregisterAuthUserAccountAction {
    readonly state: ApplicationState<UnregisterAuthUserAccountState>
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
    const { state, post } = initApplicationState({ initialState })

    const { editable, data, handler } = initEditableDataHandler<UnregisterAuthUserAccountEntry>()

    onSuccess(() => {
        editable.close()
    })

    return {
        action: {
            state,
            editable,

            data,

            onSuccess,

            async submit(): Promise<UnregisterAuthUserAccountState> {
                const element = data()
                if (!element.isLoad) {
                    return state.currentState()
                }

                return unregisterUser(material, element.data, post)
            },
        },
        handler,
    }

    function onSuccess(handler: (data: Readonly<{ loginId: LoginId }>) => void): void {
        state.subscribe((state) => {
            if (state.type === "success") {
                handler(state.data)
            }
        })
    }
}

type UnregisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: UnregisterAuthUserAccountError }>
    | Readonly<{ type: "success"; data: UnregisterAuthUserAccountEntry }>

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

    return post({ type: "success", data: user })
}

interface Post<E, S> {
    (event: E): S
}
