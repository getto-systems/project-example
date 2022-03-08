import { SearchSidebarState } from "../action"

export function sidebarExpand(state: SearchSidebarState): boolean {
    switch (state.type) {
        case "initial-sidebar":
        case "repository-error":
            return true

        case "succeed-to-load":
        case "succeed-to-save":
            return state.state.isExpand
    }
}
