import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"

export type SingleValueFilter =
    | Readonly<{ search: false }>
    | Readonly<{ search: true; value: BoardValue }>

export type MultipleValueFilter = readonly BoardValue[]

export type ReadSearchResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; value: BoardValue }>

export type SearchPageRequest = Readonly<{
    offset: BoardValue
    limit: BoardValue
}>
export type SearchPageResponse = Readonly<{
    offset: number
    limit: number
    all: number
}>
