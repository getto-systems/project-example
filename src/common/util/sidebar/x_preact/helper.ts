import { ToggleSidebarState } from "../action"

export function isSidebarExpand(state: ToggleSidebarState): boolean {
    switch (state.type) {
        case "repository-error":
            return true

        case "success":
            return state.state.isExpand
    }
}
