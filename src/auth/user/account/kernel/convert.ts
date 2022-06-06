import { AuthUserExtract, AuthUserField, TypeAuthUser } from "./data"

export function restoreAuthUserField<K extends AuthUserField>(
    value: AuthUserExtract[K],
): TypeAuthUser<K> {
    return value as TypeAuthUser<K>
}
