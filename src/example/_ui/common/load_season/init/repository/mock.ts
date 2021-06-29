import { initMemoryDB } from "../../../../../../z_details/_ui/repository/init/memory"

import { SeasonRepository } from "../../infra"

export function mockSeasonRepository(): SeasonRepository {
    return initMemoryDB()
}
