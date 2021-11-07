import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"

import { SeasonRepository } from "../../infra"

export function mockSeasonRepository(): SeasonRepository {
    return initMemoryDB()
}
