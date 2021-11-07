import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initMultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import {
    initialSearchColumnsState,
    SearchColumnsAction,
    SearchColumnsMaterial,
    SearchColumnsState,
} from "./action"
import { MultipleInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"

import { loadSearchColumns, saveSearchColumns } from "../columns/method"

import { MultipleBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"
import { SearchColumnsInfra } from "../columns/infra"

export type LoadSearchColumnsInfra = SearchColumnsInfra

export function initSearchColumnsMaterial(infra: LoadSearchColumnsInfra): SearchColumnsMaterial {
    return {
        load: loadSearchColumns(infra),
        save: saveSearchColumns(infra),
    }
}

export function initSearchColumnsAction(material: SearchColumnsMaterial): SearchColumnsAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<SearchColumnsState>
    implements SearchColumnsAction
{
    readonly initialState = initialSearchColumnsState

    readonly input: MultipleInputBoardAction

    material: SearchColumnsMaterial
    store: MultipleBoardValueStore

    constructor(material: SearchColumnsMaterial) {
        super()

        const { input, store, subscriber } = initMultipleInputBoardAction()

        this.input = input
        this.material = material
        this.store = store

        subscriber.subscribe(() => {
            material.save(store.get(), this.post)
        })

        this.terminateHook(() => {
            subscriber.terminate()
        })
    }

    load(initial: readonly string[]): Promise<SearchColumnsState> {
        return this.material.load(initial, (event) => {
            switch (event.type) {
                case "succeed-to-load":
                    this.store.set(event.columns)
                    break
            }
            return this.post(event)
        })
    }
}
