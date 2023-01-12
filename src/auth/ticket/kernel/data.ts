import { AuthPermission } from "../../user/kernel/data"

export type AuthTicket = Readonly<{
    authAt: Date
    granted: readonly AuthPermission[]
}> & { AuthTicket: never }
