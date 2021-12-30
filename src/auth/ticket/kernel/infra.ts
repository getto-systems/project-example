import { RemoteResult } from "../../../z_lib/ui/remote/infra"
import { FetchRepositoryResult, StoreRepositoryResult } from "../../../z_lib/ui/repository/infra"

import { AuthProfile, RenewAuthTicketRemoteError } from "./data"

export interface AuthProfileRepository {
    get(): Promise<FetchRepositoryResult<AuthProfile>>
    set(value: AuthProfile): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type AuthProfileRepositoryValue = Readonly<{
    authAt: string
    roles: string[]
}>

export interface RenewAuthTicketRemote {
    (): Promise<RemoteResult<AuthProfile, RenewAuthTicketRemoteError>>
}

export type AuthRemoteValue = Readonly<{
    roles: string[]
}>
