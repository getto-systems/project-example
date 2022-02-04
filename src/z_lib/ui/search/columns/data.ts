import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"

export type SearchColumns = readonly BoardValue[] & { SearchColumns: never }
