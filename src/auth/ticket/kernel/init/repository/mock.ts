import { initMemoryDB } from "../../../../../z_lib/ui/repository/init/memory"

import { AuthnRepository, AuthzRepository } from "../../infra"

export function mockAuthnRepository(): AuthnRepository {
    return initMemoryDB()
}
export function mockAuthzRepository(): AuthzRepository {
    return initMemoryDB()
}