const { invoke } = window.__TAURI__.tauri;

let grid_btn = document.getElementById("btn_randgrid");
let clear_btn = document.getElementById("btn_clear");
let solve_btn = document.getElementById("btn_solve");
grid_btn.addEventListener("click", random_grid);
clear_btn.addEventListener("click", clear_all);
solve_btn.addEventListener("click", solve);

let width_in = document.getElementById("width");
let height_in = document.getElementById("height");
let width = parseInt(width_in.value, 10);
let height = parseInt(height_in.value, 10);

const select_alg = document.getElementById("algs");
let open_nodes = document.getElementById("lbl_open");
let visit_nodes = document.getElementById("lbl_visit");

// Canvas and context for visualization
const mazeCanvas = document.getElementById("maze-canvas");
const ctx = mazeCanvas.getContext("2d");

let maze;
// Maze dimensions (adjustable)
let mazeWidth = 30;
let mazeHeight = 20;
// Cell size for drawing
let cellSize = Math.min(
	mazeCanvas.width / mazeWidth,
	mazeCanvas.height / mazeHeight
);

window.addEventListener("DOMContentLoaded", () => {
	points.start = { x: 0, y: 0 };
	points.destination = { x: 1, y: 1 };
	width_in.value = "40";
	height_in.value = "50";
	disable_btns();
	random_grid();
});

let points = {};

// Add event listener to the maze canvas
mazeCanvas.addEventListener("click", function (event) {
	// Remove any markings except walls
	for (let y = 0; y < mazeHeight; y++) {
		for (let x = 0; x < mazeWidth; x++) {
			if (
				maze[y][x] !== "block" &&
				maze[y][x] !== "start" &&
				maze[y][x] !== "destination"
			) {
				maze[y][x] = "blank";
			}
		}
	}
	let selectedAction = document.getElementById("action").value;

	// Calculate the cell coordinates based on the click position
	const rect = mazeCanvas.getBoundingClientRect();
	const mouseX = event.clientX - rect.left;
	const mouseY = event.clientY - rect.top;
	const cellX = Math.floor(mouseX / cellSize);
	const cellY = Math.floor(mouseY / cellSize);

	// Check if the clicked cell is an empty cell and not already a point
	if (
		maze[cellY][cellX] === "blank" &&
		(points.start.x !== cellX || points.start.y !== cellY) &&
		(points.destination.x !== cellX || points.destination.y !== cellY)
	) {
		switch (selectedAction) {
			case "block":
				maze[cellY][cellX] = "block";
				break;
			case "start":
				maze[points.start.x][points.start.y] = "blank";
				// Update points object with start point coordinates
				points.start.y = cellX;
				points.start.x = cellY;
				maze[cellY][cellX] = "start";
				break;
			case "dest":
				maze[points.destination.x][points.destination.y] = "blank";
				// Update points object with destination point coordinates
				points.destination.y = cellX;
				points.destination.x = cellY;
				maze[cellY][cellX] = "destination";
				break;
		}
		// Redraw the maze to reflect the updated state
		drawMaze(maze);
	} else {
		// Provide feedback to the user if attempting to place on a point or wall
		if (maze[cellY][cellX] === "block") {
			alert("You cannot place a point on a wall!");
		} else {
			alert("You cannot place points on top of each other!");
		}
	}
});

async function random_grid() {
	const context = mazeCanvas.getContext("2d");
	context.clearRect(0, 0, mazeCanvas.width, mazeCanvas.height);
	mazeHeight = parseInt(width_in.value, 10);
	mazeWidth = parseInt(height_in.value, 10);
	cellSize = Math.min(
		mazeCanvas.width / mazeWidth,
		mazeCanvas.height / mazeHeight
	);
	clear_canvas();
	maze = await invoke("make_random_grid", {
		width: parseInt(width_in.value, 10),
		height: parseInt(height_in.value, 10),
	});

	points.start.x = maze.findIndex((row) => row.includes("start"));
	points.start.y =
		maze[maze.findIndex((row) => row.includes("start"))].indexOf("start");
	points.destination.x = maze.findIndex((row) => row.includes("destination"));
	points.destination.y =
		maze[maze.findIndex((row) => row.includes("destination"))].indexOf(
			"destination"
		);

	drawMaze(maze);
}

function clear_canvas() {
	let clear = new Array(mazeHeight)
		.fill(null)
		.map(() => new Array(mazeWidth).fill("blank"));
	drawMaze(clear);
}
function clear_all() {
	const context = mazeCanvas.getContext("2d");
	context.clearRect(0, 0, mazeCanvas.width, mazeCanvas.height);
	mazeHeight = parseInt(width_in.value, 10);
	mazeWidth = parseInt(height_in.value, 10);
	cellSize = Math.min(
		mazeCanvas.width / mazeWidth,
		mazeCanvas.height / mazeHeight
	);
	let clear = new Array(mazeHeight)
		.fill(null)
		.map(() => new Array(mazeWidth).fill("blank"));
	maze = clear;
	drawMaze(clear);
}

// Function to draw the maze on the canvas
function drawMaze(maze) {
	for (let y = 0; y < mazeHeight; y++) {
		for (let x = 0; x < mazeWidth; x++) {
			const centerX = (x + 0.5) * cellSize;
			const centerY = (y + 0.5) * cellSize;
			const radius = 0.425 * cellSize; // Adjust the radius as needed

			// Draw border
			if (maze[y][x] !== "block") {
				ctx.strokeStyle = "gray";
				ctx.strokeRect(x * cellSize, y * cellSize, cellSize, cellSize);
			} else {
				ctx.fillStyle = "black"; // Wall
				ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
				ctx.strokeStyle = "#01230d";
				ctx.strokeRect(
					x * cellSize,
					y * cellSize,
					cellSize - cellSize / 10,
					cellSize - cellSize / 10
				);
			}

			if (maze[y][x] === "block") {
				ctx.fillStyle = "black"; // Wall
				ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
				ctx.fillStyle = "#01230d"; // Wall
				ctx.fillRect(
					x * cellSize,
					y * cellSize,
					cellSize - cellSize / 10,
					cellSize - cellSize / 10
				);
			} else if (maze[y][x] === "start") {
				ctx.fillStyle = "white"; // Empty cell
				ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
				ctx.fillStyle = "Blue"; // Start point
				ctx.beginPath();
				ctx.arc(centerX, centerY, radius, 0, 2 * Math.PI);
				ctx.fill();
				ctx.fillStyle = "White"; // Start point
				ctx.beginPath();
				ctx.arc(centerX, centerY, radius - radius / 10, 0, 2 * Math.PI);
				ctx.fill();
				ctx.fillStyle = "#1C6758"; // Start point
				ctx.beginPath();
				ctx.arc(centerX, centerY, radius - radius / 5, 0, 2 * Math.PI);
				ctx.fill();

				ctx.font = "10px Arial";
				ctx.fillStyle = "white";
				// Calculate the width of the text
				const textWidth = ctx.measureText("S").width;
				// Calculate the x-coordinate to center the text horizontally within the cell
				const textX = x * cellSize + (cellSize - textWidth) / 2;
				// Calculate the y-coordinate to center the text vertically within the cell
				const textY = y * cellSize + cellSize / 2 + 10 / 4;
				// Draw the text
				ctx.fillText("S", textX, textY);
			} else if (maze[y][x] === "destination") {
				ctx.fillStyle = "white"; // Empty cell
				ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
				ctx.fillStyle = "Black";
				ctx.beginPath();
				ctx.arc(centerX, centerY, radius, 0, 2 * Math.PI);
				ctx.fill();
				ctx.fillStyle = "Yellow";
				ctx.beginPath();
				ctx.arc(centerX, centerY, radius - radius / 10, 0, 2 * Math.PI);
				ctx.fill();
				ctx.fillStyle = "#BE0000"; // Destination point
				ctx.beginPath();
				ctx.arc(centerX, centerY, radius - radius / 5, 0, 2 * Math.PI);
				ctx.fill();

				ctx.font = "10px Arial";
				ctx.fillStyle = "white";
				// Calculate the width of the text
				const textWidth = ctx.measureText("D").width;
				// Calculate the x-coordinate to center the text horizontally within the cell
				const textX = x * cellSize + (cellSize - textWidth) / 2;
				// Calculate the y-coordinate to center the text vertically within the cell
				const textY = y * cellSize + cellSize / 2 + 10 / 4;
				// Draw the text
				ctx.fillText("D", textX, textY);
			} else if (maze[y][x] === "blank") {
				ctx.fillStyle = "white"; // Empty cell
				ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
			}
		}
	}
	// disable_btns();
}
async function solve() {
	disable_btns();
	open_nodes.textContent = "";
	visit_nodes.textContent = "";
	clear_canvas();
	drawMaze(maze);
	let alg = select_alg.value;

	let algorithm;
	if (alg === "dfs") {
		algorithm = await invoke("dfs_solve", {
			arr: maze,
			start: [points.start.x, points.start.y],
		});
	} else if (alg === "bfs") {
		algorithm = await invoke("bfs_solve", {
			arr: maze,
			start: [points.start.x, points.start.y],
		});
	} else {
		algorithm = await invoke("a_star_solve", {
			arr: maze,
			start: [points.start.x, points.start.y],
			dest: [points.destination.x, points.destination.y],
		});
	}
	if (algorithm === null || algorithm[0].length === 0) {
		alert("No path found");
		enable_btns();
	} else if (algorithm.length === 3) {
		// a_start
		const path = algorithm[0];
		const visited = algorithm[1];
		const time = algorithm[2];
		console.log("Time : ", time);
		await draw_visited(visited);
		draw_path(path);
	} else {
		// dfs and bfs
		const path = algorithm[0];
		const time = algorithm[1];
		console.log("Time : ", time);
		draw_path(path);
	}
	// enable_btns();
}

const timer = (ms) => new Promise((res) => setTimeout(res, ms));

async function draw_path(path) {
	// console.log(path);
	for (let i = 1; i < path.length + 1; i++) {
		open_nodes.textContent = i;
		const x = path[i - 1][1];
		const y = path[i - 1][0];

		const centerX = (x + 0.5) * cellSize;
		const centerY = (y + 0.5) * cellSize;
		const radius = 0.425 * cellSize; // Adjust the radius as needed

		ctx.fillStyle = "#9999ff";
		ctx.beginPath();
		ctx.arc(centerX, centerY, radius, 0, 2 * Math.PI);
		ctx.fill();

		ctx.font = "10px Arial";
		ctx.fillStyle = "black";
		// Calculate the width of the text
		const textWidth = ctx.measureText(i).width;
		// Calculate the x-coordinate to center the text horizontally within the cell
		const textX = x * cellSize + (cellSize - textWidth) / 2;
		// Calculate the y-coordinate to center the text vertically within the cell
		const textY = y * cellSize + cellSize / 2 + 10 / 4;
		// Draw the text
		ctx.fillText(i, textX, textY);

		await timer(50);
	}
	enable_btns();
}
async function draw_visited(visited) {
	// console.log(visited);
	for (let i = 1; i < visited.length + 1; i++) {
		visit_nodes.textContent = i;
		const x = visited[i - 1][1];
		const y = visited[i - 1][0];

		const centerX = (x + 0.5) * cellSize;
		const centerY = (y + 0.5) * cellSize;
		const radius = 0.425 * cellSize; // Adjust the radius as needed

		ctx.fillStyle = "#99cc00";

		ctx.beginPath();
		ctx.arc(centerX, centerY, radius, 0, 2 * Math.PI);
		ctx.fill();

		await timer(50);
	}
}
function disable_btns() {
	grid_btn.toggleAttribute("disabled");
	clear_btn.toggleAttribute("disabled");
	solve_btn.toggleAttribute("disabled");
}
function enable_btns() {
	grid_btn.disabled = false;
	clear_btn.disabled = false;
	solve_btn.disabled = false;
}
