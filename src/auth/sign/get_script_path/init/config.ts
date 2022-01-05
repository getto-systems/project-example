import { env } from "../../../../y_environment/ui/env";
import { GetScriptPathConfig, SecureServerURL } from "../infra";


export function newGetScriptPathConfig(): GetScriptPathConfig {
    return {
        secureServerURL: env.secureServerURL as SecureServerURL,
    };
}
