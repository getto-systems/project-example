import { RemoteCommonError } from "../../../../../../z_lib/ui/remote/data"

export type ChangeResetTokenDestinationError = ChangeResetTokenDestinationRemoteError

export type ChangeResetTokenDestinationRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "conflict" }>
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
