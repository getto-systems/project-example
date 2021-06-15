import { initMemoryDB } from "../../../../../z_details/_ui/repository/infra/memory"

import { MenuExpandRepository } from "../../infra"

export function mockMenuExpandRepository(): MenuExpandRepository {
    return initMemoryDB()
}
