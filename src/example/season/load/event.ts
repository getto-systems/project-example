import { RepositoryError } from "../../../z_lib/ui/repository/data"

import { Season } from "../kernel/data"

export type LoadSeasonEvent =
    | Readonly<{
          type: "succeed-to-load"
          season: Season
          default: boolean
          availableSeasons: Season[]
      }>
    | Readonly<{ type: "failed-to-load"; err: RepositoryError }>
