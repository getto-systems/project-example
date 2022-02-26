export type SearchResponseResult<R> =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; response: R }>
export type SearchState<R> =
    | Readonly<{ type: "initial-search" }>
    | ((
          | Readonly<{ type: "try-to-search" }>
          | Readonly<{ type: "take-longtime-to-search" }>
          | Readonly<{ type: "failed-to-search" }>
          | Readonly<{ type: "succeed-to-search"; response: R }>
      ) &
          Readonly<{
              previousResponse?: R
          }>)
export function searchResponse<R>(state: SearchState<R>): SearchResponseResult<R> {
    switch (state.type) {
        case "initial-search":
        case "failed-to-search":
            return { found: false }

        case "try-to-search":
        case "take-longtime-to-search":
            if (!state.previousResponse) {
                return { found: false }
            }
            return { found: true, response: state.previousResponse }

        case "succeed-to-search":
            return { found: true, response: state.response }
    }
}
