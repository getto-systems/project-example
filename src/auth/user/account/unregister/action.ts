import { checkTakeLongtime } from "../../../../common/util/timer/helper"

import { Atom, initAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../common/util/load/data"
import { LoadableListAtomUpdater } from "../../../../common/util/list/action"
import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../../common/util/board/editable/action"

import { UnregisterAuthUserAccountRemote } from "./infra"
import { WaitTime } from "../../../../common/util/config/infra"

import { UnregisterAuthUserAccountError } from "./data"
import { LoginId } from "../../login_id/kernel/data"
import { AuthUserAccount } from "../kernel/data"
import { ConnectState } from "../../../../common/util/connect/data"

export interface UnregisterAuthUserAccountAction {
    readonly state: Atom<UnregisterAuthUserAccountState>
    readonly connect: Atom<ConnectState>
    readonly editable: EditableBoardAction

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
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
    material: UnregisterAuthUserAccountMaterial,
): UnregisterAuthUserAccountAction {
    const unregister = initAtom({ initialState })
    async function unregisterWithCurrentState(): Promise<UnregisterAuthUserAccountState> {
        const element = data.currentState()
        if (!element.isLoad) {
            return unregister.state.currentState()
        }

        return unregisterAuthUserAccount(material, element.data, unregister.post)
    }

    const editable = initEditableBoardAction()

    unregister.state.subscribe((state) => {
        if (state.type === "success") {
            updater.update((list) => list.filter((item) => item.loginId !== state.data.loginId))
            editable.close()
        }
    })

    const connect = mapAtom(unregister.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: unregister.state,
        connect,
        editable,

        submit: unregisterWithCurrentState,
    }
}

type UnregisterUserEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: UnregisterAuthUserAccountError }>
    | Readonly<{ type: "success"; data: UnregisterAuthUserAccountEntry }>

async function unregisterAuthUserAccount<S>(
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
