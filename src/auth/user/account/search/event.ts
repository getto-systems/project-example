import { SearchAuthUserAccountError, SearchAuthUserAccountRemoteResponse } from "./data"

export type SearchAuthUserAccountEvent =
    | Readonly<{ type: "try-to-search" }>
    | Readonly<{ type: "take-longtime-to-search" }>
    | Readonly<{ type: "failed-to-search"; err: SearchAuthUserAccountError }>
    | Readonly<{ type: "succeed-to-search"; response: SearchAuthUserAccountRemoteResponse }>
