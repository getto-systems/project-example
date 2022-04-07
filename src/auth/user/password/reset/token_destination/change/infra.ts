import { RemoteResult } from "../../../../../../z_lib/ui/remote/infra"

import { LoginId } from "../../../../login_id/input/data"
import { ResetTokenDestination } from "../kernel/data"
import { ChangeResetTokenDestinationRemoteError } from "./data"

export interface ChangeResetTokenDestinationRemote {
    (
        user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
        fields: ResetTokenDestination,
    ): Promise<ChangeResetTokenDestinationRemoteResult>
}

export type ChangeResetTokenDestinationRemoteResult = RemoteResult<
    true,
    ChangeResetTokenDestinationRemoteError
>
