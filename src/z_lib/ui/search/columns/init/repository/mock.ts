import { initMemoryDB } from "../../../../repository/init/memory"

import { SearchColumnsRepository } from "../../infra"

export function mockSearchColumnsRepository(): SearchColumnsRepository {
    return initMemoryDB()
}
