const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  console.log('hi there')
  const gridData = [
      ['start', 'blank', 'blank', 'blank', 'block', 'blank'],
      ['block', 'blank', 'blank', 'blank', 'blank', 'blank'],
      ['blank', 'blank', 'block', 'block', 'block', 'blank'],
      ['blank', 'block', 'blank', 'blank', 'block', 'blank'],
      ['blank', 'blank', 'blank', 'blank', 'destination', 'block'],
    ];
    console.log(gridData)
    console.log("dfs: ")
    let dfs = invoke('dfs_solve', {arr:gridData})
    console.log(dfs)

    console.log("---------------------")
    console.log("bfs: ")
    let bfs = invoke('bfs_solve', {arr:gridData})
    console.log(bfs)
    
    console.log("---------------------")
    console.log("a star: ")
    let astar = invoke('a_star_solve', {arr:gridData})
    console.log(astar)



  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

