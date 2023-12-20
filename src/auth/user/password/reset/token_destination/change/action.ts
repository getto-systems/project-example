import { checkTakeLongtime, ticker } from "../../../../../../common/util/timer/helper"

import { Atom, initAtom, mapAtom } from "../../../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loaded } from "../../../../../../common/util/load/data"
import { EditableBoardAction } from "../../../../../../common/util/board/editable/action"
import { ValidateBoardState } from "../../../../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../../../../common/util/board/observe/action"
import { composeModifyFieldBoard } from "../../../../../../common/util/board/field/action"
import { LoadableListAtomUpdater } from "../../../../../../common/util/list/action"
import { initResetTokenDestinationField, ResetTokenDestinationField } from "../input/field/action"

import { WaitTime } from "../../../../../../common/util/config/infra"
import { ChangeResetTokenDestinationRemote } from "./infra"

import { ConvertBoardResult } from "../../../../../../common/util/board/kernel/data"
import { AuthUserAccount } from "../../../../account/kernel/data"
import { ResetTokenDestination, resetTokenDestinationTypeVariants } from "../kernel/data"
import { ChangeResetTokenDestinationError } from "./data"
import { ConnectState } from "../../../../../../common/util/connect/data"

export interface ChangeResetTokenDestinationAction {
    readonly state: Atom<ChangeResetTokenDestinationState>
    readonly connect: Atom<ConnectState>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
    readonly editable: EditableBoardAction

    readonly destination: ResetTokenDestinationField

    reset(): void
    submit(): Promise<ChangeResetTokenDestinationState>
}

export type ChangeResetTokenDestinationState = ChangeDestinationEvent

const initialState: ChangeResetTokenDestinationState = { type: "initial" }

export type ChangeResetTokenDestinationMaterial = Readonly<{
    infra: ChangeResetTokenDestinationInfra
    config: ChangeResetTokenDestinationConfig
}>

export type ChangeResetTokenDestinationInfra = Readonly<{
    changeDestinationRemote: ChangeResetTokenDestinationRemote
}>

export type ChangeResetTokenDestinationConfig = Readonly<{
    takeLongtimeThreshold: WaitTime
    resetToInitialTimeout: WaitTime
}>

export function initChangeResetTokenDestinationAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
    material: ChangeResetTokenDestinationMaterial,
): ChangeResetTokenDestinationAction {
    const modify = initAtom({ initialState })
    async function modifyWithCurrentState(): Promise<ChangeResetTokenDestinationState> {
        const element = data.currentState()
        if (!element.isLoad) {
            return modify.state.currentState()
        }

        const fields = currentFields()
        if (!fields.valid) {
            return modify.state.currentState()
        }
        return changeDestination(material, element.data, fields.value, modify.post)
    }

    const destinationOptions = initAtom({
        initialState: loadState_loaded(resetTokenDestinationTypeVariants),
    })

    const destination = initResetTokenDestinationField(destinationOptions.state)

    const currentFields = (): ConvertBoardResult<ResetTokenDestination> => {
        return destination[0].validate.currentState()
    }

    const { editable, validate, observe, reset } = composeModifyFieldBoard(data, [
        [destination, (data: AuthUserAccount) => data.resetTokenDestination],
    ])

    modify.state.subscribe((state) => {
        if (state.type === "success") {
            updater.update((list) =>
                list.map((item) => {
                    return item.loginId === state.data.loginId ? state.data : item
                }),
            )
        }
    })

    const connect = mapAtom(modify.state, (state): ConnectState => {
        if (state.type === "try") {
            return { isConnecting: true, hasTakenLongtime: state.hasTakenLongtime }
        } else {
            return { isConnecting: false }
        }
    })

    return {
        state: modify.state,
        connect,
        validate,
        observe,
        editable,

        destination: destination[0],

        reset,
        submit: modifyWithCurrentState,
    }
}

type ChangeDestinationEvent =
    | Readonly<{ type: "try"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed"; err: ChangeResetTokenDestinationError }>
    | Readonly<{ type: "success"; data: AuthUserAccount }>
    | Readonly<{ type: "initial" }>

async function changeDestination<S>(
    { infra, config }: ChangeResetTokenDestinationMaterial,
    user: AuthUserAccount,
    fields: ResetTokenDestination,
    post: Post<ChangeDestinationEvent, S>,
): Promise<S> {
    post({ type: "try", hasTakenLongtime: false })

    const { changeDestinationRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(
        changeDestinationRemote(user, fields),
        config.takeLongtimeThreshold,
        () => post({ type: "try", hasTakenLongtime: true }),
    )
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    post({ type: "success", data: { ...user, resetTokenDestination: fields } })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
