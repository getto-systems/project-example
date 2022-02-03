import { RepositoryError } from "../../../z_lib/ui/repository/data"

export type FocusSeasonEvent =
    | Readonly<{ type: "succeed-to-focus" }>
    | Readonly<{ type: "invalid-season" }>
    | Readonly<{ type: "failed-to-focus"; err: RepositoryError }>
