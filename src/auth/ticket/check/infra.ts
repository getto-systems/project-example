import { RemoteCommonError } from "../../../z_lib/ui/remote/data"
import { RemoteResult } from "../../../z_lib/ui/remote/infra"
import { AuthTicket } from "../kernel/data"

export interface CheckAuthTicketRemote {
    (): Promise<RemoteResult<AuthTicket, RemoteCommonError>>
}
