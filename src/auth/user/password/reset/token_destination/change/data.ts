import { RemoteCommonError } from "../../../../../../z_lib/ui/remote/data"

export type ChangeResetTokenDestinationError =
    | Readonly<{ type: "validation-error" }>
    | ChangeResetTokenDestinationRemoteError

export type ChangeResetTokenDestinationRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "conflict" }>
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid-destination-type" }>
    | Readonly<{ type: "invalid-email" }>
