import { FetchRepositoryResult, StoreRepositoryResult } from "../../../z_lib/ui/repository/infra"

import { AuthTicket } from "./data"

export interface AuthTicketRepository {
    get(): Promise<FetchRepositoryResult<AuthTicket>>
    set(value: AuthTicket): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type AuthTicketRepositoryValue = Readonly<{
    authAt: string
    grantedRoles: readonly string[]
}>
