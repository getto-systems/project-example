import { FetchRepositoryResult, StoreRepositoryResult } from "../../../common/util/repository/infra"

import { AuthTicket } from "./data"

export interface AuthTicketRepository {
    get(): Promise<FetchRepositoryResult<AuthTicket>>
    set(value: AuthTicket): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type AuthTicketRepositoryValue = Readonly<{
    authAt: string
    granted: readonly string[]
}>
