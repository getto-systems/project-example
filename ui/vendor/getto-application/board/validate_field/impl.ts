import { ValidateBoardFieldInfra } from "./infra"

import { ConvertBoardFieldMethod } from "./method"

interface Convert {
    <T, E>(infra: ValidateBoardFieldInfra<T, E>): ConvertBoardFieldMethod<T, E>
}
export const convertBoardField: Convert = (infra) => (post) => {
    const { converter } = infra
    const result = converter()
    if (result.valid) {
        post({ valid: true })
    } else {
        post(result)
    }
    return result
}
