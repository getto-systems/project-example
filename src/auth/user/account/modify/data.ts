import { RemoteCommonError } from "../../../../common/util/remote/data"
import { AuthPermission } from "../../kernel/data"
import { TypeAuthUser } from "../kernel/data"

export type ModifyAuthUserAccountFields = Readonly<{
    memo: TypeAuthUser<"memo">
    granted: readonly AuthPermission[]
}>

export type ModifyAuthUserAccountError = ModifyAuthUserAccountRemoteError

export type ModifyAuthUserAccountRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "conflict" }>
    | Readonly<{ type: "not-found" }>
    | Readonly<{ type: "invalid" }>
