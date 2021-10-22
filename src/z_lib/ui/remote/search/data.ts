import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export type SearchPageRequest = Readonly<{
    offset: BoardValue
    limit: BoardValue
}>
export type SearchPageResponse = Readonly<{
    offset: number
    limit: number
    all: number
}>
