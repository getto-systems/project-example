import { initMemoryDB } from "../../../../../../z_details/_ui/repository/infra/memory"

import { AuthnRepository } from "../../infra"

export function mockAuthnRepository(): AuthnRepository {
    return initMemoryDB()
}
