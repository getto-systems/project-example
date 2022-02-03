import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../../ui/vendor/getto-application/action/action"

import { initInputSeasonAction, InputSeasonAction } from "../input/action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime } from "../../../z_lib/ui/config/infra"
import { LoadSeasonState } from "../load/action"
import { seasonBoardConverter, seasonToBoardValue } from "../kernel/convert"
import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"

export interface FocusSeasonAction extends StatefulApplicationAction<FocusSeasonState> {
    readonly season: InputSeasonAction

    open(): Promise<FocusSeasonState>
    focus(): Promise<FocusSeasonState>
}

export type FocusSeasonMaterial = Readonly<{
    infra: FocusSeasonInfra
    config: FocusSeasonConfig
}>
export type FocusSeasonInfra = Readonly<{
    seasonRepository: SeasonRepository
    clock: Clock
}>
export type FocusSeasonConfig = Readonly<{
    focusSeasonExpire: ExpireTime
}>

export type FocusSeasonState =
    | Readonly<{ type: "initial-focus" }>
    | Readonly<{ type: "edit-season" }>
    | FocusSeasonEvent

export const initialFocusSeasonState: FocusSeasonState = { type: "initial-focus" }

export function initFocusSeasonAction(
    material: FocusSeasonMaterial,
    loadState: Promise<LoadSeasonState>,
): FocusSeasonAction {
    return new Action(material, loadState)
}

class Action extends AbstractStatefulApplicationAction<FocusSeasonState> {
    readonly initialState = initialFocusSeasonState

    readonly season: InputSeasonAction

    material: FocusSeasonMaterial

    field: { (): BoardValue }

    constructor(material: FocusSeasonMaterial, loadState: Promise<LoadSeasonState>) {
        super()

        const season = initInputSeasonAction()

        this.season = season.input

        loadState.then((state) => {
            switch (state.type) {
                case "succeed-to-load":
                    if (!state.default) {
                        season.set(seasonToBoardValue(state.season))
                    }
                    return
            }
        })

        this.material = material
        this.field = () => season.get()
    }

    async focus(): Promise<FocusSeasonState> {
        return focusSeason(this.material, this.field(), this.post)
    }
    async open(): Promise<FocusSeasonState> {
        return this.post({ type: "edit-season" })
    }
}

type FocusSeasonEvent =
    | Readonly<{ type: "succeed-to-focus" }>
    | Readonly<{ type: "invalid-season" }>
    | Readonly<{ type: "failed-to-focus"; err: RepositoryError }>

async function focusSeason<S>(
    { infra, config }: FocusSeasonMaterial,
    value: BoardValue,
    post: Post<FocusSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository } = infra

    const convertResult = seasonBoardConverter(value)
    if (!convertResult.valid) {
        return post({ type: "invalid-season" })
    }

    if (convertResult.default) {
        const result = await seasonRepository.remove()
        if (!result.success) {
            return post({ type: "failed-to-focus", err: result.err })
        }
        return post({ type: "succeed-to-focus" })
    }

    const result = await seasonRepository.set({
        season: convertResult.season,
        expires: clock.now().getTime() + config.focusSeasonExpire.expire_millisecond,
    })
    if (!result.success) {
        return post({ type: "failed-to-focus", err: result.err })
    }

    return post({ type: "succeed-to-focus" })
}

interface Post<E, S> {
    (event: E): S
}
