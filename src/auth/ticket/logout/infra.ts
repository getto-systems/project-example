import { RemoteResult } from "../../../common/util/remote/infra"

import { RemoteCommonError } from "../../../common/util/remote/data"

export interface LogoutRemote {
    (): Promise<RemoteResult<true, RemoteCommonError>>
}
