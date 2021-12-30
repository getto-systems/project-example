import { RemoteResult } from "../../../z_lib/ui/remote/infra"
import { AuthProfileRepository } from "../kernel/infra"

import { LogoutRemoteError } from "./data"

export type LogoutInfra = Readonly<{
    profileRepository: AuthProfileRepository
    logoutRemote: LogoutRemote
}>

export interface LogoutRemote {
    (): Promise<RemoteResult<true, LogoutRemoteError>>
}
