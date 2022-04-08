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
    | Readonly<{ type: "success"; state: SearchSidebarExpand }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

export type SearchSidebarInfra = Readonly<{
    sidebarRepository: SearchSidebarRepository
}>

export function initSearchSidebarAction(
    infra: SearchSidebarInfra,
    state: SearchSidebarExpand,
): SearchSidebarAction {
    return new Action(infra, state)
}

class Action
    extends AbstractStatefulApplicationAction<SearchSidebarState>
    implements SearchSidebarAction
{
    initialState: SearchSidebarState

    infra: SearchSidebarInfra

    constructor(infra: SearchSidebarInfra, state: SearchSidebarExpand) {
        super({
            ignite: () => this.load(),
        })

        this.initialState = { type: "success", state }

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
        return this.post({ type: "success", state })
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

        return this.post({ type: "success", state: sidebarResult.value })
    }
}
