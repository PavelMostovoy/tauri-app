const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  await invoke("open_print");
}
async function save_to_db() {
  greetMsgEl.textContent = await invoke("save_to_db", { name: greetInputEl.value });
}

async function save_to_pdf(){
  window.print();
}

window.greet = greet;
window.save_to_db = save_to_db;
window.save_to_pdf = save_to_pdf;
