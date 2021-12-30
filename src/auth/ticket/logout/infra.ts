import { RemoteResult } from "../../../z_lib/ui/remote/infra"

import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export interface LogoutRemote {
    (): Promise<RemoteResult<true, RemoteCommonError>>
}
