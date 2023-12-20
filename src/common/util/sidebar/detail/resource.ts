import { RepositoryOutsideFeature } from "../../repository/feature"

import { newSearchSidebarRepository } from "./sidebar_repository"

import { initToggleSidebarAction, ToggleSidebarAction } from "../action"

export function newToggleSidebarAction(
    feature: RepositoryOutsideFeature,
    key: string,
): ToggleSidebarAction {
    return initToggleSidebarAction(
        {
            sidebarRepository: newSearchSidebarRepository(feature, key),
        },
        { isExpand: true },
    )
}
