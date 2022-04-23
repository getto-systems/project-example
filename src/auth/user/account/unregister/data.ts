import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

export type UnregisterAuthUserAccountError = UnregisterAuthUserAccountRemoteError

export type UnregisterAuthUserAccountRemoteError = RemoteCommonError | Readonly<{ type: "invalid" }>
