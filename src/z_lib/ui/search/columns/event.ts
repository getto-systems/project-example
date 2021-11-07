import { RepositoryError } from "../../repository/data"
import { SearchColumns } from "./data"

export type LoadSearchColumnsEvent =
    | Readonly<{ type: "succeed-to-load"; columns: SearchColumns }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

export type SaveSearchColumnsEvent =
    | Readonly<{ type: "succeed-to-save"; columns: SearchColumns }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
