import { RepositoryConverter } from "../../repository/infra"

import { SearchSidebarExpand } from "./data"

export const searchSidebarRepositoryConverter: RepositoryConverter<
    SearchSidebarExpand,
    SearchSidebarExpand
> = {
    toRepository: (value) => value,
    fromRepository: (value) => {
        return { valid: true, value }
    },
}
