import { RemoteCommonError } from "../../../../common/util/remote/data"

export type UnregisterAuthUserAccountError = UnregisterAuthUserAccountRemoteError

export type UnregisterAuthUserAccountRemoteError = RemoteCommonError | Readonly<{ type: "invalid" }>
