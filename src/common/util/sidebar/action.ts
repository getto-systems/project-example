import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"

import { ToggleSidebarRepository } from "./infra"

import { SidebarExpand } from "./data"
import { RepositoryError } from "../repository/data"

export interface ToggleSidebarAction {
    readonly state: ApplicationState<ToggleSidebarState>
    fold(): Promise<ToggleSidebarState>
    expand(): Promise<ToggleSidebarState>
}

export type ToggleSidebarState =
    | Readonly<{ type: "success"; state: SidebarExpand }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

export type ToggleSidebarInfra = Readonly<{
    sidebarRepository: ToggleSidebarRepository
}>

export function initToggleSidebarAction(
    infra: ToggleSidebarInfra,
    initialExpand: SidebarExpand,
): ToggleSidebarAction {
    const { state, post } = initApplicationState({
        initialState: { type: "success", state: initialExpand },
        ignite: async (): Promise<ToggleSidebarState> => {
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

        async fold(): Promise<ToggleSidebarState> {
            return set({ isExpand: false })
        },
        async expand(): Promise<ToggleSidebarState> {
            return set({ isExpand: true })
        },
    }

    async function set(state: SidebarExpand): Promise<ToggleSidebarState> {
        const { sidebarRepository } = infra
        const result = await sidebarRepository.set(state)
        if (!result.success) {
            return post({ type: "repository-error", err: result.err })
        }
        return post({ type: "success", state })
    }
}
