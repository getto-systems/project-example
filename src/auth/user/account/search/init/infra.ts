import { SearchAuthUserAccountInfra } from "../action"
import { newSearchAuthUserAccountRemote } from "./search_remote"

export function newSearchAuthUserAccountInfra(): SearchAuthUserAccountInfra {
    return {
        searchRemote: newSearchAuthUserAccountRemote(),
    }
}
