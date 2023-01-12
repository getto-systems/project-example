import { RepositoryConverter } from "../repository/infra"

import { SidebarExpand } from "./data"

export const searchSidebarRepositoryConverter: RepositoryConverter<SidebarExpand, SidebarExpand> = {
    toRepository: (value) => value,
    fromRepository: (value) => {
        return { valid: true, value }
    },
}
