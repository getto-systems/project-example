import { initMemoryDB } from "../../../../../../z_details/_ui/repository/infra/memory"

import { AuthnRepository, AuthzRepository } from "../../infra"

export function mockAuthnRepository(): AuthnRepository {
    return initMemoryDB()
}
export function mockAuthzRepository(): AuthzRepository {
    return initMemoryDB()
}
