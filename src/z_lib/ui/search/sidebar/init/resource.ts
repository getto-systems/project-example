import { RepositoryOutsideFeature } from "../../../repository/feature"

import { newSearchSidebarRepository } from "./sidebar_repository"

import { initSearchSidebarAction, SearchSidebarAction } from "../action"

export function newSearchSidebarAction(
    feature: RepositoryOutsideFeature,
    key: string,
): SearchSidebarAction {
    return initSearchSidebarAction(
        {
            sidebarRepository: newSearchSidebarRepository(feature, key),
        },
        { isExpand: true },
    )
}
