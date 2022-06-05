import { ValidateSelectError, ValidateTextError } from "../data"

export function textValidationError(err: readonly ValidateTextError[]): readonly string[] {
    return err.map((err) => {
        switch (err.type) {
            case "empty":
                return "入力してください"

            case "too-long":
                return `${err.maxLength}文字以内で入力してください`

            case "invalid-email":
                return "正しいメールアドレスを入力してください"
        }
    })
}

export function selectValidationError(err: ValidateSelectError): readonly string[] {
    switch (err.type) {
        case "not-selected":
            return ["選択してください"]
    }
}
