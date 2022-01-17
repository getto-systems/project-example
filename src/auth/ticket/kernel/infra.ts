import { RemoteCommonError } from "../../../z_lib/ui/remote/data"
import { RemoteResult } from "../../../z_lib/ui/remote/infra"
import { FetchRepositoryResult, StoreRepositoryResult } from "../../../z_lib/ui/repository/infra"

import { AuthTicket } from "./data"

export interface AuthTicketRepository {
    get(): Promise<FetchRepositoryResult<AuthTicket>>
    set(value: AuthTicket): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type AuthTicketRepositoryValue = Readonly<{
    authAt: string
    roles: readonly string[]
}>

export interface RenewAuthTicketRemote {
    (): Promise<RemoteResult<AuthTicket, RemoteCommonError>>
}

export type AuthRemoteValue = Readonly<{
    roles: readonly string[]
}>
