const fs = require("node:fs/promises");
const path = require("node:path");
const markdown = require("./markdown");
const jsonReporter = require("./json");

const RESULT_DIR = path.resolve(__dirname, "..", "results");

async function ensureDir() {
  await fs.mkdir(RESULT_DIR, { recursive: true });
}

async function writeReport(results, format) {
  await ensureDir();
  const timestamp = Date.now();
  const ext = format === "json" ? "json" : "md";

  let content;
  let filename;
  if (format === "json") {
    content = jsonReporter.render(results);
    filename = `audit-report-${timestamp}.json`;
  } else {
    content = markdown.render(results);
    filename = `audit-report-${timestamp}.md`;
  }

  const filePath = path.join(RESULT_DIR, filename);
  await fs.writeFile(filePath, content, "utf8");
  return filePath;
}

async function writeBoth(results) {
  const mdPath = await writeReport(results, "markdown");
  const jsonPath = await writeReport(results, "json");
  return { markdown: mdPath, json: jsonPath };
}

module.exports = { writeReport, writeBoth };
