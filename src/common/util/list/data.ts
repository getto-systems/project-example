export type DetectFocusListKeyResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; key: string }>

export type ListSearchedResult<T, M, E> =
    | Readonly<{ type: "success"; response: ListSearchedData<T, M> }>
    | Readonly<{ type: "failed"; err: E }>

export type ListSearchedData<T, M> = Readonly<{ list: readonly T[] }> & M
