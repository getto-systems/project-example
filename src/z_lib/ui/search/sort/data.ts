export type SearchSort<K> = Readonly<{ key: K; order: SearchSortOrder }>
export type SearchSortOrder = "normal" | "reverse"

export type ReadSearchSortKeyResult<K> =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; key: K }>
