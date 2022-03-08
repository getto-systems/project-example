import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../z_vendor/getto-application/action/action"

import { SearchSidebarRepository } from "./infra"

import { SearchSidebarExpand } from "./data"
import { RepositoryError } from "../../repository/data"

export interface SearchSidebarAction extends StatefulApplicationAction<SearchSidebarState> {
    fold(): Promise<SearchSidebarState>
    expand(): Promise<SearchSidebarState>
}

export type SearchSidebarState =
    | Readonly<{ type: "initial-sidebar"; state: SearchSidebarExpand }>
    | Readonly<{ type: "succeed-to-save"; state: SearchSidebarExpand }>
    | Readonly<{ type: "succeed-to-load"; state: SearchSidebarExpand }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

const initialState: SearchSidebarState = { type: "initial-sidebar", state: { isExpand: true } }

export type SearchSidebarInfra = Readonly<{
    sidebarRepository: SearchSidebarRepository
}>

export function initSearchSidebarAction(infra: SearchSidebarInfra): SearchSidebarAction {
    return new Action(infra)
}

class Action
    extends AbstractStatefulApplicationAction<SearchSidebarState>
    implements SearchSidebarAction
{
    readonly initialState = initialState

    infra: SearchSidebarInfra

    constructor(infra: SearchSidebarInfra) {
        super({
            ignite: () => this.load(),
        })

        this.infra = infra
    }

    async fold(): Promise<SearchSidebarState> {
        return this.set({ isExpand: false })
    }
    async expand(): Promise<SearchSidebarState> {
        return this.set({ isExpand: true })
    }
    async set(state: SearchSidebarExpand): Promise<SearchSidebarState> {
        const { sidebarRepository } = this.infra
        const result = await sidebarRepository.set(state)
        if (!result.success) {
            return this.post({ type: "repository-error", err: result.err })
        }
        return this.post({ type: "succeed-to-save", state })
    }

    async load(): Promise<SearchSidebarState> {
        const { sidebarRepository } = this.infra

        const sidebarResult = await sidebarRepository.get()
        if (!sidebarResult.success) {
            return this.post({ type: "repository-error", err: sidebarResult.err })
        }
        if (!sidebarResult.found) {
            return this.post(this.currentState())
        }

        return this.post({ type: "succeed-to-load", state: sidebarResult.value })
    }
}
