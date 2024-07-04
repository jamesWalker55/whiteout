import { invoke } from "@tauri-apps/api";

export async function greet(name: string): Promise<string> {
  return await invoke("greet", { name });
}
