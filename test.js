const fs = require("fs");
const path = require("path");

const BASE_DIR = `X:\\OneDrive\\programmering\\rs\\spotify_data_explorer\\data\\full`;

const files = fs.readdirSync(BASE_DIR);

const full_files = files.map((f) => path.join(BASE_DIR, f));

const into_arr = [];

console.time("PARSING FILES");

for (const file_path of full_files) {
  console.time(`    - ${file_path}`);
  const rv = JSON.parse(fs.readFileSync(file_path), "utf8");
  into_arr.push(rv);
  console.timeEnd(`    - ${file_path}`);
}

console.timeEnd("PARSING FILES");
