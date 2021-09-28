import { initMemoryDB } from "../../../../../z_lib/ui/repository/init/memory"

import { MenuExpandRepository } from "../../infra"

export function mockMenuExpandRepository(): MenuExpandRepository {
    return initMemoryDB()
}
