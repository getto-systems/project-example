import { SearchUserAccountError } from "./data"

export type SearchUserAccountEvent =
    | Readonly<{ type: "try-to-search" }>
    | Readonly<{ type: "take-longtime-to-search" }>
    | Readonly<{ type: "failed-to-search"; err: SearchUserAccountError }>
    | Readonly<{ type: "succeed-to-search" }> // TODO page: SearchPage, users: UserAccount[]
