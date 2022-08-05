import {
    ApplicationState,
    initApplicationStateAction,
    StatefulApplicationAction,
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

class Action implements SearchSidebarAction {
    readonly infra: SearchSidebarInfra
    readonly state: ApplicationState<SearchSidebarState>
    readonly post: (state: SearchSidebarState) => SearchSidebarState

    constructor(infra: SearchSidebarInfra, initialExpand: SearchSidebarExpand) {
        const { state, post } = initApplicationStateAction({
            initialState: { type: "success", state: initialExpand },
            ignite: () => this.load(),
        })
        this.infra = infra
        this.state = state
        this.post = post
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
            return this.post(this.state.currentState())
        }

        return this.post({ type: "success", state: sidebarResult.value })
    }
}
