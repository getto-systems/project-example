export type SingleValueFilter =
    | Readonly<{ search: false }>
    | Readonly<{ search: true; value: string }>

export type MultipleValueFilter = readonly string[]

export type ReadSearchResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; value: string }>

export type SearchPageRequest = Readonly<{
    offset: string
    limit: string
}>
export type SearchPageResponse = Readonly<{
    offset: number
    limit: number
    all: number
}>
