import { LoginId } from "./data"

export function restoreLoginId(loginId: string): LoginId {
    return loginId as LoginId
}

export function emptyLoginId(): LoginId {
    return restoreLoginId("")
}
