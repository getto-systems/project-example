export type SearchSort = Readonly<{
    key: string
    order: SearchSortOrder
}>
export type SearchSortOrder = "normal" | "reverse"

export const normalSearchSortOrder: SearchSortOrder = "normal"
