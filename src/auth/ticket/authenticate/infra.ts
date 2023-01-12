import { RemoteCommonError } from "../../../common/util/remote/data"
import { RemoteResult } from "../../../common/util/remote/infra"
import { AuthTicket } from "../kernel/data"

export interface CheckAuthTicketRemote {
    (): Promise<RemoteResult<AuthTicket, RemoteCommonError>>
}
