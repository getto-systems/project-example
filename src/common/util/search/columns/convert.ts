import { RepositoryConverter } from "../../repository/infra"
import { SearchColumnsRepositoryValue } from "./infra"

export const searchColumnsRepositoryConverter: RepositoryConverter<
    readonly string[],
    SearchColumnsRepositoryValue
> = {
    toRepository: (value) => value,
    fromRepository: (value) => {
        return {
            valid: true,
            value,
        }
    },
}
