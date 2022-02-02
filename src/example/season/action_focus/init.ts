import { AbstractStatefulApplicationAction } from "../../../../ui/vendor/getto-application/action/action"

import { initInputSeasonAction } from "../input/action_input/init"

import {
    initialFocusSeasonState,
    FocusSeasonAction,
    FocusSeasonMaterial,
    FocusSeasonState,
} from "./action"
import { LoadSeasonState } from "../action_load/action"
import { InputSeasonAction } from "../input/action_input/action"

import { focusSeason } from "../focus/method"

import { FocusSeasonInfra } from "../focus/infra"

import { seasonToBoardValue } from "../kernel/convert"
import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { Season } from "../kernel/data"

export type FocusSeasonActionInfra = Readonly<{
    focusSeason: FocusSeasonInfra
}>

export function initFocusSeasonAction(
    infra: FocusSeasonActionInfra,
    loadState: Promise<LoadSeasonState>,
): FocusSeasonAction {
    return new Action(
        {
            focusSeason: focusSeason(infra.focusSeason),
        },
        loadState,
    )
}

class Action extends AbstractStatefulApplicationAction<FocusSeasonState> {
    readonly initialState = initialFocusSeasonState

    readonly season: InputSeasonAction

    material: FocusSeasonMaterial

    availableSeasons: Season[] = []
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
        return this.material.focusSeason(this.field(), this.post)
    }
    async open(): Promise<FocusSeasonState> {
        return this.post({ type: "edit-season" })
    }
}
