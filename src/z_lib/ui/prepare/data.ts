export type PrepareElementState<T> =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "loaded"; data: T }>
