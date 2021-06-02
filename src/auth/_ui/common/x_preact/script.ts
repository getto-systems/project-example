import { ScriptPath } from "../secure/get_script_path/data"

export function appendScript(scriptPath: ScriptPath, setup: Setup<HTMLScriptElement>): void {
    const script = document.createElement("script")
    script.src = scriptPath
    setup(script)
    document.body.appendChild(script)
}

interface Setup<T> {
    (element: T): void
}
