import { RepositoryError } from "../../../../../ui/vendor/getto-application/infra/repository/data"

import { Season } from "./data"

export type LoadSeasonEvent =
    | Readonly<{ type: "succeed-to-load"; value: Season }>
    | Readonly<{ type: "failed-to-load"; err: RepositoryError }>
