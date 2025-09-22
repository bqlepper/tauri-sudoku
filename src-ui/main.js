const { invoke } = window.__TAURI__.tauri;

let bqlMsgElement;

async function bqlTest() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    bqlMsgElement.textContent = 'Hellow from BQL!';
    //bqlMsgElement.textContent = `Kee pressd: ${e.key}, Cell number: ${cellNumber}`;
    //bqlMsgElement.textContent = await invoke("greet", { name: e.key });
}

window.addEventListener("DOMContentLoaded", () => {
    bqlMsgElement = document.querySelector("#bql-msg");
    bqlMsgElement.textContent = "Ready to go!";
});
