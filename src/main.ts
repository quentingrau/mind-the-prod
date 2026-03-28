import { listen } from "@tauri-apps/api/event";

async function main() {
  // Setup listeners and wait for them to be ready
  await listen("dangerous", () => {
    const body = document.querySelector("body");
    body!.classList.add("dangerous");
  });

  await listen("safe", () => {
    const body = document.querySelector("body");
    body!.classList.remove("dangerous");
  });
}

main();
