import { initMemoryDB } from "../../../../../../z_details/_ui/repository/infra/memory"

import { SeasonRepository } from "../../infra"

export function mockSeasonRepository(): SeasonRepository {
    return initMemoryDB()
}
