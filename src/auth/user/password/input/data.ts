import { ValidateTextError } from "../../../../z_lib/ui/validate/data"
import { ConvertBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"

export type Password = string & { Password: never }

export type PasswordCharacterState = Readonly<{ multiByte: boolean }>

export type ConvertPasswordResult = ConvertBoardFieldResult<Password, readonly ValidateTextError[]>

export type ResetToken = string & { ResetToken: never }
