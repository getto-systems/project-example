import {
    ApplicationState,
    initApplicationState,
} from "../../../../z_vendor/getto-application/action/action"

import { SearchSidebarRepository } from "./infra"

import { SearchSidebarExpand } from "./data"
import { RepositoryError } from "../../repository/data"

export interface SearchSidebarAction {
    readonly state: ApplicationState<SearchSidebarState>
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
    initialExpand: SearchSidebarExpand,
): SearchSidebarAction {
    const { state, post } = initApplicationState({
        initialState: { type: "success", state: initialExpand },
        ignite: async (): Promise<SearchSidebarState> => {
            const { sidebarRepository } = infra

            const sidebarResult = await sidebarRepository.get()
            if (!sidebarResult.success) {
                return post({ type: "repository-error", err: sidebarResult.err })
            }
            if (!sidebarResult.found) {
                return post(state.currentState())
            }

            return post({ type: "success", state: sidebarResult.value })
        },
    })

    return {
        state,

        async fold(): Promise<SearchSidebarState> {
            return set({ isExpand: false })
        },
        async expand(): Promise<SearchSidebarState> {
            return set({ isExpand: true })
        },
    }

    async function set(state: SearchSidebarExpand): Promise<SearchSidebarState> {
        const { sidebarRepository } = infra
        const result = await sidebarRepository.set(state)
        if (!result.success) {
            return post({ type: "repository-error", err: result.err })
        }
        return post({ type: "success", state })
    }
}
